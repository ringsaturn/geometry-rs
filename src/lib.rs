#![doc = include_str!("../README.md")]

mod next_after;

use crate::next_after::NextAfter;
use rtree_rs::{RTree, Rect as RTreeRect};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point<T = f64> {
    pub x: T,
    pub y: T,
}

/// A coordinate stored as an integer with an application-defined scale.
pub type I32Point = Point<i32>;

/// Coordinate storage for [`Polygon`].
///
/// The polygon's bounding box, acceleration indexes and raycast all operate in
/// "storage space": the query point is multiplied by the polygon's scale once
/// per containment check, and segment endpoints are converted with
/// [`CoordStorage::to_f64`] in registers. `f64::from(i32)` is exact (`i32`
/// fits within `f64`'s 53-bit mantissa), so integer storage loses no
/// precision, and `x * 1.0` is an exact identity, so `f64` storage behaves
/// bit-identically to a non-generic implementation.
pub trait CoordStorage: Copy + PartialOrd {
    /// Convert a stored coordinate to `f64`.
    /// Identity for `f64`; exact `f64::from` for `i32`.
    fn to_f64(self) -> f64;
    /// Convert a stored coordinate to `i64` for the opt-in integer raycast.
    /// Only meaningful for integer storage.
    fn to_i64(self) -> i64;
}

impl CoordStorage for f64 {
    #[inline(always)]
    fn to_f64(self) -> f64 {
        self
    }

    #[inline(always)]
    fn to_i64(self) -> i64 {
        self as i64
    }
}

impl CoordStorage for i32 {
    #[inline(always)]
    fn to_f64(self) -> f64 {
        f64::from(self)
    }

    #[inline(always)]
    fn to_i64(self) -> i64 {
        i64::from(self)
    }
}

pub trait ContainsPoint<Q> {
    fn contains_point(&self, point: Q) -> bool;
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum I32RaycastMode {
    #[default]
    Float,
    Integer,
}

/// A compact polygon whose coordinates are scaled integers.
///
/// This is [`Polygon`] with `i32` storage: eight bytes per stored point, with
/// full support for the acceleration indexes in [`PolygonBuildOptions`].
pub type I32Polygon = Polygon<i32>;

#[inline]
fn ring_contains_point_integer<T: CoordStorage>(
    ring: &[Point<T>],
    point: Point<i64>,
    allow_on_edge: bool,
) -> bool {
    let mut inside = false;
    for pair in ring.windows(2) {
        let ax = pair[0].x.to_i64();
        let ay = pair[0].y.to_i64();
        let bx = pair[1].x.to_i64();
        let by = pair[1].y.to_i64();
        let cross = (bx - ax) * (point.y - ay) - (by - ay) * (point.x - ax);
        if cross == 0
            && point.x >= ax.min(bx)
            && point.x <= ax.max(bx)
            && point.y >= ay.min(by)
            && point.y <= ay.max(by)
        {
            return allow_on_edge;
        }
        if (ay > point.y) != (by > point.y) && (cross > 0) == (by > ay) {
            inside = !inside;
        }
    }
    inside
}

#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pub min: Point,
    pub max: Point,
}

impl Rect {
    pub fn contains_point(&self, p: Point) -> bool {
        return p.x >= self.min.x && p.x <= self.max.x && p.y >= self.min.y && p.y <= self.max.y;
    }

    pub fn intersects_rect(&self, other: Rect) -> bool {
        if self.min.y > other.max.y || self.max.y < other.min.y {
            return false;
        }
        if self.min.x > other.max.x || self.max.x < other.min.x {
            return false;
        }
        return true;
    }

    pub fn nw(&self) -> Point {
        Point {
            x: self.min.x,
            y: self.max.y,
        }
    }

    pub fn sw(&self) -> Point {
        Point {
            x: self.min.x,
            y: self.min.y,
        }
    }

    pub fn se(&self) -> Point {
        Point {
            x: self.max.x,
            y: self.min.y,
        }
    }

    pub fn ne(&self) -> Point {
        Point {
            x: self.max.x,
            y: self.max.y,
        }
    }

    pub fn south(&self) -> Segment {
        Segment {
            a: self.sw(),
            b: self.se(),
        }
    }

    pub fn east(&self) -> Segment {
        Segment {
            a: self.se(),
            b: self.ne(),
        }
    }

    pub fn north(&self) -> Segment {
        Segment {
            a: self.ne(),
            b: self.nw(),
        }
    }

    pub fn west(&self) -> Segment {
        Segment {
            a: self.nw(),
            b: self.sw(),
        }
    }

    pub fn segment_at(&self, index: i64) -> Segment {
        match index {
            0 => self.south(),
            1 => self.east(),
            2 => self.north(),
            3 => self.west(),
            _ => self.south(), // TODO(ringsaturn): raise err
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PolygonBuildOptions {
    pub enable_rtree: bool,
    pub enable_compressed_quad: bool,
    pub enable_y_stripes: bool,
    pub rtree_min_segments: usize,
}

impl Default for PolygonBuildOptions {
    fn default() -> Self {
        Self {
            enable_rtree: false,
            enable_compressed_quad: true,
            enable_y_stripes: false,
            rtree_min_segments: 64,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct YStripesBuildStats {
    pub segment_count: usize,
    pub stripe_count: usize,
    pub assigned_item_count: usize,
    pub max_bucket_len: usize,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RingBuildStats {
    pub segment_count: usize,
    pub below_threshold: bool,
    pub used_rtree: bool,
    pub used_compressed_quad: bool,
    pub used_y_stripes: bool,
    pub y_stripes: Option<YStripesBuildStats>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PolygonIndexStats {
    pub exterior: RingBuildStats,
    pub holes: Vec<RingBuildStats>,
}

struct RingIndex {
    x_min: f64,
    x_max: f64,
    seg_count: usize,
    rtree: Option<RTree<2, f64, usize>>,
    compressed_quad: Option<CompressedQuadIndex>,
    y_stripes: Option<YStripesIndex>,
}

impl RingIndex {
    fn search_candidates<T: CoordStorage>(
        &self,
        ring: &[Point<T>],
        point_y: f64,
        out: &mut Vec<usize>,
    ) {
        // The common configurations enable one index. Avoid allocating the
        // deduplication bitmap and temporary vectors when no merge is needed.
        match (
            self.rtree.as_ref(),
            self.y_stripes.as_ref(),
            self.compressed_quad.as_ref(),
        ) {
            (None, Some(index), None) => {
                index.search(ring, point_y, out);
                return;
            }
            (Some(tree), None, None) => {
                let query = RTreeRect::new([self.x_min, point_y], [self.x_max, point_y]);
                out.extend(
                    tree.search(query)
                        .map(|item| *item.data)
                        .filter(|&idx| idx < self.seg_count),
                );
                return;
            }
            (None, None, Some(index)) => {
                let query_rect = Rect {
                    min: Point {
                        x: self.x_min,
                        y: point_y,
                    },
                    max: Point {
                        x: self.x_max,
                        y: point_y,
                    },
                };
                index.search_intersects(query_rect, out);
                return;
            }
            _ => {}
        }

        let query_rect = Rect {
            min: Point {
                x: self.x_min,
                y: point_y,
            },
            max: Point {
                x: self.x_max,
                y: point_y,
            },
        };

        let mut seen = vec![false; self.seg_count];

        if let Some(tree) = self.rtree.as_ref() {
            let query = RTreeRect::new([self.x_min, point_y], [self.x_max, point_y]);
            for item in tree.search(query) {
                let idx = *item.data;
                if idx < self.seg_count && !seen[idx] {
                    seen[idx] = true;
                    out.push(idx);
                }
            }
        }

        if let Some(index) = self.y_stripes.as_ref() {
            let mut tmp = Vec::new();
            index.search(ring, point_y, &mut tmp);
            for idx in tmp {
                if idx < self.seg_count && !seen[idx] {
                    seen[idx] = true;
                    out.push(idx);
                }
            }
        }

        if let Some(index) = self.compressed_quad.as_ref() {
            let mut tmp = Vec::new();
            index.search_intersects(query_rect, &mut tmp);
            for idx in tmp {
                if idx < self.seg_count && !seen[idx] {
                    seen[idx] = true;
                    out.push(idx);
                }
            }
        }
    }
}

#[derive(Clone, Copy, Default)]
struct YStripe {
    start: u32,
    count: u32,
}

struct YStripesIndex {
    min_y: f64,
    height: f64,
    stripes: Vec<YStripe>,
    indexes: Vec<u32>,
}

impl YStripesIndex {
    fn build<T: CoordStorage>(
        ring: &[Point<T>],
        seg_rects: &[Rect],
    ) -> Option<(Self, YStripesBuildStats)> {
        if seg_rects.is_empty() {
            return None;
        }

        let mut min_y = seg_rects[0].min.y;
        let mut max_y = seg_rects[0].max.y;
        for rect in seg_rects.iter().copied() {
            min_y = min_y.min(rect.min.y);
            max_y = max_y.max(rect.max.y);
        }

        let mut stripe_count = calc_y_stripe_count(ring, seg_rects.len());
        if stripe_count == 0 {
            stripe_count = 1;
        }

        let height = max_y - min_y;
        let mut counts = vec![0usize; stripe_count];
        for rect in seg_rects.iter().copied() {
            let (start, end) = stripe_bounds_for_rect(rect, min_y, height, stripe_count);
            for stripe in start..=end {
                counts[stripe] += 1;
            }
        }

        let mut stripes = vec![YStripe::default(); stripe_count];
        let mut offset = 0usize;
        for (stripe, count) in stripes.iter_mut().zip(&counts) {
            stripe.start = u32::try_from(offset).ok()?;
            stripe.count = 0;
            offset += *count;
        }
        // Segment indices and stripe offsets are stored as u32.
        if u32::try_from(offset).is_err() || u32::try_from(seg_rects.len()).is_err() {
            return None;
        }

        let mut indexes = vec![0u32; offset];
        for (idx, rect) in seg_rects.iter().copied().enumerate() {
            let (start, end) = stripe_bounds_for_rect(rect, min_y, height, stripe_count);
            for stripe_index in start..=end {
                let stripe = &mut stripes[stripe_index];
                indexes[(stripe.start + stripe.count) as usize] = idx as u32;
                stripe.count += 1;
            }
        }

        let mut stats = YStripesBuildStats {
            segment_count: seg_rects.len(),
            stripe_count,
            assigned_item_count: indexes.len(),
            max_bucket_len: counts.into_iter().max().unwrap_or(0),
        };
        if stats.stripe_count == 0 {
            stats.stripe_count = 1;
        }

        Some((
            Self {
                min_y,
                height,
                stripes,
                indexes,
            },
            stats,
        ))
    }

    fn search<T: CoordStorage>(&self, ring: &[Point<T>], y: f64, out: &mut Vec<usize>) {
        if self.height == 0.0 {
            if y != self.min_y {
                return;
            }
        } else if y < self.min_y || y > self.min_y + self.height {
            return;
        }

        let stripe_index = if self.height == 0.0 {
            0
        } else {
            let raw = ((y - self.min_y) / self.height * self.stripes.len() as f64).floor() as isize;
            raw.clamp(0, self.stripes.len() as isize - 1) as usize
        };
        let stripe = self.stripes[stripe_index];
        let start = stripe.start as usize;
        let end = start + stripe.count as usize;
        for idx in &self.indexes[start..end] {
            let idx = *idx as usize;
            // Recompute the segment's y range from its endpoints instead of
            // storing it; the endpoints are about to be fetched for the
            // raycast anyway.
            let a_y = ring[idx].y.to_f64();
            let b_y = ring[idx + 1].y.to_f64();
            let (seg_min_y, seg_max_y) = if a_y <= b_y { (a_y, b_y) } else { (b_y, a_y) };
            if y >= seg_min_y && y <= seg_max_y {
                out.push(idx);
            }
        }
    }
}

fn stripe_bounds_for_rect(
    rect: Rect,
    min_y: f64,
    height: f64,
    stripe_count: usize,
) -> (usize, usize) {
    if stripe_count <= 1 || height == 0.0 {
        return (0, 0);
    }

    let last = stripe_count - 1;
    let start = (((rect.min.y - min_y) / height) * stripe_count as f64).floor() as isize;
    let end = (((rect.max.y - min_y) / height) * stripe_count as f64).floor() as isize;
    (
        start.clamp(0, last as isize) as usize,
        end.clamp(0, last as isize) as usize,
    )
}

fn calc_ring_area_and_perimeter<T: CoordStorage>(ring: &[Point<T>]) -> (f64, f64) {
    let seg_count = ring_segment_count(ring);
    if seg_count == 0 {
        return (0.0, 0.0);
    }

    // Computed in storage space; the stripe-count score below is the
    // isoperimetric quotient, which is invariant under uniform scaling.
    let mut signed_area = 0.0;
    let mut perimeter = 0.0;
    for i in 0..seg_count {
        let a = Point {
            x: ring[i].x.to_f64(),
            y: ring[i].y.to_f64(),
        };
        let b = Point {
            x: ring[i + 1].x.to_f64(),
            y: ring[i + 1].y.to_f64(),
        };
        signed_area += a.x * b.y - b.x * a.y;
        perimeter += ((b.x - a.x).powi(2) + (b.y - a.y).powi(2)).sqrt();
    }
    (signed_area.abs() * 0.5, perimeter)
}

fn calc_y_stripe_count<T: CoordStorage>(ring: &[Point<T>], seg_count: usize) -> usize {
    let (area, perimeter) = calc_ring_area_and_perimeter(ring);
    let mut score = 0.0;
    if perimeter > 0.0 {
        score = (area * std::f64::consts::PI * 4.0) / (perimeter * perimeter);
    }
    ((seg_count as f64 * score).floor() as usize).max(32)
}

const Q_MAX_ITEMS: usize = 12;
const Q_MAX_DEPTH: usize = 64;

#[derive(Default)]
struct QuadNode {
    split: bool,
    items: Vec<usize>,
    quads: [Option<Box<QuadNode>>; 4],
}

impl QuadNode {
    fn new() -> Self {
        Self {
            split: false,
            items: Vec::new(),
            quads: [None, None, None, None],
        }
    }
}

struct CompressedQuadIndex {
    bounds: Rect,
    seg_rects: Vec<Rect>,
    data: Vec<u8>,
}

impl CompressedQuadIndex {
    fn build<T: CoordStorage>(ring: &[Point<T>]) -> Option<Self> {
        let seg_count = ring_segment_count(ring);
        if seg_count == 0 {
            return None;
        }

        let mut min_x = ring[0].x.to_f64();
        let mut min_y = ring[0].y.to_f64();
        let mut max_x = min_x;
        let mut max_y = min_y;

        for p in ring.iter().take(seg_count) {
            let x = p.x.to_f64();
            let y = p.y.to_f64();
            if x < min_x {
                min_x = x;
            }
            if y < min_y {
                min_y = y;
            }
            if x > max_x {
                max_x = x;
            }
            if y > max_y {
                max_y = y;
            }
        }

        let bounds = Rect {
            min: Point { x: min_x, y: min_y },
            max: Point { x: max_x, y: max_y },
        };

        let mut seg_rects = Vec::with_capacity(seg_count);
        for i in 0..seg_count {
            seg_rects.push(segment_at_for_slice(ring, i).rect());
        }

        let mut root = QuadNode::new();
        for i in 0..seg_rects.len() {
            insert_quad_node(&mut root, bounds, &seg_rects, i, 0);
        }

        let mut data = Vec::with_capacity(seg_rects.len() * 2);
        compress_quad_node(&root, &mut data);

        Some(Self {
            bounds,
            seg_rects,
            data,
        })
    }

    fn search_intersects(&self, query: Rect, out: &mut Vec<usize>) {
        if !self.bounds.intersects_rect(query) {
            return;
        }
        let _ = self.search_intersects_from(0, self.bounds, query, out);
    }

    fn search_intersects_from(
        &self,
        mut addr: usize,
        bounds: Rect,
        query: Rect,
        out: &mut Vec<usize>,
    ) -> Option<usize> {
        let (nitems, next_addr) = read_uvarint(&self.data, addr)?;
        addr = next_addr;

        let mut last: usize = 0;
        for _ in 0..nitems {
            let (delta, next_addr) = read_uvarint(&self.data, addr)?;
            addr = next_addr;
            last = last.checked_add(delta as usize)?;
            let seg_rect = self.seg_rects.get(last)?;
            if seg_rect.intersects_rect(query) {
                out.push(last);
            }
        }

        let split = *self.data.get(addr)?;
        addr += 1;
        if split == 0 {
            return Some(addr);
        }
        if split != 1 {
            return None;
        }

        for q in 0..4 {
            let (qsize, next_addr) = read_uvarint(&self.data, addr)?;
            addr = next_addr;
            if qsize == 0 {
                continue;
            }

            let qsize = usize::try_from(qsize).ok()?;
            let qbounds = quad_bounds(bounds, q);
            let child_start = addr;
            let child_end = child_start.checked_add(qsize)?;
            if child_end > self.data.len() {
                return None;
            }

            if qbounds.intersects_rect(query) {
                let _ = self.search_intersects_from(child_start, qbounds, query, out)?;
            }
            addr = child_end;
        }

        Some(addr)
    }
}

fn ring_segment_count<T>(ring: &[Point<T>]) -> usize {
    ring.len().saturating_sub(1)
}

// Rings are stored closed (first point repeated at the end), so segment `i`
// is `ring[i]..ring[i+1]` and the segment count is `len - 1`. The YStripes
// build and its query-time y-range recomputation both rely on this numbering.
fn segment_at_for_slice<T: CoordStorage>(ring: &[Point<T>], index: usize) -> Segment {
    Segment {
        a: Point {
            x: ring[index].x.to_f64(),
            y: ring[index].y.to_f64(),
        },
        b: Point {
            x: ring[index + 1].x.to_f64(),
            y: ring[index + 1].y.to_f64(),
        },
    }
}

fn build_ring_index<T: CoordStorage>(
    ring: &[Point<T>],
    options: &PolygonBuildOptions,
) -> (Option<RingIndex>, RingBuildStats) {
    let seg_count = ring_segment_count(ring);
    let index_requested =
        options.enable_rtree || options.enable_compressed_quad || options.enable_y_stripes;
    let mut stats = RingBuildStats {
        segment_count: seg_count,
        below_threshold: index_requested && seg_count < options.rtree_min_segments,
        ..RingBuildStats::default()
    };

    if !index_requested || ring.is_empty() || seg_count < options.rtree_min_segments {
        return (None, stats);
    }

    let mut x_min = ring[0].x.to_f64();
    let mut x_max = x_min;
    for p in ring.iter().take(seg_count) {
        let x = p.x.to_f64();
        if x < x_min {
            x_min = x;
        }
        if x > x_max {
            x_max = x;
        }
    }

    let rtree = if options.enable_rtree {
        stats.used_rtree = true;
        let mut tree: RTree<2, f64, usize> = RTree::new();
        for i in 0..seg_count {
            let seg_rect = segment_at_for_slice(ring, i).rect();
            tree.insert(
                RTreeRect::new(
                    [seg_rect.min.x, seg_rect.min.y],
                    [seg_rect.max.x, seg_rect.max.y],
                ),
                i,
            );
        }
        Some(tree)
    } else {
        None
    };

    let compressed_quad = if options.enable_compressed_quad {
        match CompressedQuadIndex::build(ring) {
            Some(index) => {
                stats.used_compressed_quad = true;
                Some(index)
            }
            None => None,
        }
    } else {
        None
    };

    let (y_stripes, y_stripes_stats) = if options.enable_y_stripes {
        let mut seg_rects = Vec::with_capacity(seg_count);
        for i in 0..seg_count {
            seg_rects.push(segment_at_for_slice(ring, i).rect());
        }
        match YStripesIndex::build(ring, &seg_rects) {
            Some((index, stripe_stats)) => {
                stats.used_y_stripes = true;
                (Some(index), Some(stripe_stats))
            }
            None => (None, None),
        }
    } else {
        (None, None)
    };
    stats.y_stripes = y_stripes_stats;

    if rtree.is_none() && compressed_quad.is_none() && y_stripes.is_none() {
        return (None, stats);
    }

    (
        Some(RingIndex {
            x_min,
            x_max,
            seg_count,
            rtree,
            compressed_quad,
            y_stripes,
        }),
        stats,
    )
}

fn rings_contains_point<T: CoordStorage>(
    ring: &[Point<T>],
    ring_index: Option<&RingIndex>,
    point: Point,
    allow_on_edge: bool,
) -> bool {
    let mut inside: bool = false;

    if let Some(index) = ring_index {
        let mut candidates = Vec::new();
        index.search_candidates(ring, point.y, &mut candidates);
        for i in candidates {
            let seg = segment_at_for_slice(ring, i);
            let res: RaycastResult = raycast(&seg, point);
            if res.on {
                inside = allow_on_edge;
                break;
            }
            if res.inside {
                inside = !inside;
            }
        }
        return inside;
    }

    for pair in ring.windows(2) {
        let seg = Segment {
            a: Point {
                x: pair[0].x.to_f64(),
                y: pair[0].y.to_f64(),
            },
            b: Point {
                x: pair[1].x.to_f64(),
                y: pair[1].y.to_f64(),
            },
        };

        // The ray is horizontal and unbounded in x, so intersecting the
        // segment's rect reduces to a y-range check; f64::min/max keep it
        // branchless.
        let min_y = seg.a.y.min(seg.b.y);
        let max_y = seg.a.y.max(seg.b.y);
        if point.y < min_y || point.y > max_y {
            continue;
        }

        let res: RaycastResult = raycast(&seg, point);
        if res.on {
            inside = allow_on_edge;
            break;
        }
        if res.inside {
            inside = !inside;
        }
    }

    return inside;
}

fn choose_quad(bounds: Rect, rect: Rect) -> Option<usize> {
    let mid_x = (bounds.min.x + bounds.max.x) / 2.0;
    let mid_y = (bounds.min.y + bounds.max.y) / 2.0;

    if rect.max.x < mid_x {
        if rect.max.y < mid_y {
            return Some(2);
        }
        if rect.min.y < mid_y {
            return None;
        }
        return Some(0);
    }

    if rect.min.x < mid_x {
        return None;
    }

    if rect.max.y < mid_y {
        return Some(3);
    }
    if rect.min.y < mid_y {
        return None;
    }
    Some(1)
}

fn quad_bounds(mut bounds: Rect, q: usize) -> Rect {
    let center_x = (bounds.min.x + bounds.max.x) / 2.0;
    let center_y = (bounds.min.y + bounds.max.y) / 2.0;

    match q {
        0 => {
            bounds.min.y = center_y;
            bounds.max.x = center_x;
        }
        1 => {
            bounds.min.x = center_x;
            bounds.min.y = center_y;
        }
        2 => {
            bounds.max.x = center_x;
            bounds.max.y = center_y;
        }
        3 => {
            bounds.min.x = center_x;
            bounds.max.y = center_y;
        }
        _ => {}
    }
    bounds
}

fn insert_quad_node(
    node: &mut QuadNode,
    bounds: Rect,
    seg_rects: &[Rect],
    item: usize,
    depth: usize,
) {
    if depth == Q_MAX_DEPTH {
        node.items.push(item);
        return;
    }

    let item_rect = seg_rects[item];
    if node.split {
        if let Some(q) = choose_quad(bounds, item_rect) {
            let qbounds = quad_bounds(bounds, q);
            if node.quads[q].is_none() {
                node.quads[q] = Some(Box::new(QuadNode::new()));
            }
            if let Some(quad) = node.quads[q].as_deref_mut() {
                insert_quad_node(quad, qbounds, seg_rects, item, depth + 1);
            }
        } else {
            node.items.push(item);
        }
        return;
    }

    if node.items.len() == Q_MAX_ITEMS {
        let existing = std::mem::take(&mut node.items);
        node.split = true;
        for i in existing {
            let rect = seg_rects[i];
            if let Some(q) = choose_quad(bounds, rect) {
                let qbounds = quad_bounds(bounds, q);
                if node.quads[q].is_none() {
                    node.quads[q] = Some(Box::new(QuadNode::new()));
                }
                if let Some(quad) = node.quads[q].as_deref_mut() {
                    insert_quad_node(quad, qbounds, seg_rects, i, depth + 1);
                }
            } else {
                node.items.push(i);
            }
        }
        insert_quad_node(node, bounds, seg_rects, item, depth);
        return;
    }

    node.items.push(item);
}

fn append_uvarint(dst: &mut Vec<u8>, mut x: u64) {
    while x >= 0x80 {
        dst.push((x as u8 & 0x7f) | 0x80);
        x >>= 7;
    }
    dst.push(x as u8);
}

fn read_uvarint(data: &[u8], mut addr: usize) -> Option<(u64, usize)> {
    let mut x: u64 = 0;
    let mut shift = 0;

    loop {
        let b = *data.get(addr)?;
        addr += 1;

        if shift == 70 {
            return None;
        }

        x |= ((b & 0x7f) as u64) << shift;
        if b < 0x80 {
            return Some((x, addr));
        }
        shift += 7;
    }
}

fn compress_quad_node(node: &QuadNode, dst: &mut Vec<u8>) {
    let mut items = node.items.clone();
    items.sort_unstable();

    append_uvarint(dst, items.len() as u64);
    let mut last = 0usize;
    for item in items {
        append_uvarint(dst, (item - last) as u64);
        last = item;
    }

    if !node.split {
        dst.push(0);
        return;
    }

    dst.push(1);
    for q in 0..4 {
        if let Some(child) = node.quads[q].as_deref() {
            let mut child_bytes = Vec::new();
            compress_quad_node(child, &mut child_bytes);
            append_uvarint(dst, child_bytes.len() as u64);
            dst.extend_from_slice(&child_bytes);
        } else {
            append_uvarint(dst, 0);
        }
    }
}

/// A polygon whose coordinates are stored as `T` (`f64` degrees by default,
/// or scaled `i32`, see [`I32Polygon`]).
///
/// The bounding box, the acceleration indexes and the raycast all operate in
/// storage space: [`Polygon::contains_point`] multiplies the query point by
/// the polygon's scale exactly once, and segment endpoints are converted to
/// `f64` in registers during the raycast.
pub struct Polygon<T: CoordStorage = f64> {
    exterior: Vec<Point<T>>,
    holes: Vec<Vec<Point<T>>>,
    // Storage-space bounding box, kept in storage type so the bbox reject
    // scan touches half the memory for i32 polygons.
    min: Point<T>,
    max: Point<T>,
    scale: f64,
    raycast_mode: I32RaycastMode,
    options: PolygonBuildOptions,
    exterior_index: Option<RingIndex>,
    hole_indexes: Vec<Option<RingIndex>>,
    index_stats: PolygonIndexStats,
}

impl<T: CoordStorage> Polygon<T> {
    fn compute_bounds(exterior: &[Point<T>]) -> (Point<T>, Point<T>) {
        let mut min = exterior[0];
        let mut max = exterior[0];

        for p in exterior.iter() {
            if p.x < min.x {
                min.x = p.x;
            }
            if p.y < min.y {
                min.y = p.y;
            }
            if p.x > max.x {
                max.x = p.x;
            }
            if p.y > max.y {
                max.y = p.y;
            }
        }

        (min, max)
    }

    fn build(
        exterior: Vec<Point<T>>,
        holes: Vec<Vec<Point<T>>>,
        scale: f64,
        raycast_mode: I32RaycastMode,
        options: Option<PolygonBuildOptions>,
    ) -> Self {
        let (min, max) = Self::compute_bounds(&exterior);
        let mut poly = Self {
            exterior,
            holes,
            min,
            max,
            scale,
            raycast_mode,
            options: options.unwrap_or_default(),
            exterior_index: None,
            hole_indexes: Vec::new(),
            index_stats: PolygonIndexStats::default(),
        };
        poly.rebuild_cache();
        poly
    }

    fn rebuild_cache(&mut self) {
        let (min, max) = Self::compute_bounds(&self.exterior);
        self.min = min;
        self.max = max;
        let (exterior_index, exterior_stats) = build_ring_index(&self.exterior, &self.options);
        self.exterior_index = exterior_index;
        self.index_stats.exterior = exterior_stats;

        let hole_indexes_and_stats: Vec<(Option<RingIndex>, RingBuildStats)> = self
            .holes
            .iter()
            .map(|hole| build_ring_index(hole, &self.options))
            .collect();
        let (hole_indexes, hole_stats): (Vec<Option<RingIndex>>, Vec<RingBuildStats>) =
            hole_indexes_and_stats.into_iter().unzip();
        self.hole_indexes = hole_indexes;
        self.index_stats.holes = hole_stats;
    }

    /// Point-In-Polygon check, the normal way.
    /// It's most used algorithm implementation, port from Go's [geojson]
    ///
    /// [geojson]: https://github.com/tidwall/geojson
    fn contains_point_normal(&self, p: Point) -> bool {
        if !rings_contains_point(&self.exterior, self.exterior_index.as_ref(), p, false) {
            return false;
        }

        for (hole, hole_index) in self.holes.iter().zip(self.hole_indexes.iter()) {
            if rings_contains_point(hole, hole_index.as_ref(), p, false) {
                return false;
            }
        }

        return true;
    }

    /// Do point-in-polygon search. `p` is in unscaled (degree) coordinates.
    pub fn contains_point(&self, p: Point) -> bool {
        match self.raycast_mode {
            I32RaycastMode::Float => self.contains_point_float(p),
            I32RaycastMode::Integer => self.contains_point_integer(p),
        }
    }

    fn contains_point_float(&self, p: Point) -> bool {
        // `x * 1.0` is an exact identity under IEEE 754, so f64 polygons
        // (scale == 1.0) behave bit-identically to the pre-generic code.
        let scaled = Point {
            x: p.x * self.scale,
            y: p.y * self.scale,
        };
        // Same truth table as Rect::contains_point; the storage-type bounds
        // are converted in registers.
        if !(scaled.x >= self.min.x.to_f64()
            && scaled.x <= self.max.x.to_f64()
            && scaled.y >= self.min.y.to_f64()
            && scaled.y <= self.max.y.to_f64())
        {
            return false;
        }

        return self.contains_point_normal(scaled);
    }

    // Opt-in integer cross-product raycast; snaps the query to the storage
    // grid and ignores acceleration indexes.
    fn contains_point_integer(&self, p: Point) -> bool {
        if !p.x.is_finite() || !p.y.is_finite() {
            return false;
        }
        let scaled = Point::<i64> {
            x: (p.x * self.scale).round() as i64,
            y: (p.y * self.scale).round() as i64,
        };
        if scaled.x < self.min.x.to_i64()
            || scaled.x > self.max.x.to_i64()
            || scaled.y < self.min.y.to_i64()
            || scaled.y > self.max.y.to_i64()
            || !ring_contains_point_integer(&self.exterior, scaled, false)
        {
            return false;
        }
        !self
            .holes
            .iter()
            .any(|ring| ring_contains_point_integer(ring, scaled, false))
    }
    pub fn exterior(&self) -> &[Point<T>] {
        &self.exterior
    }

    pub fn holes(&self) -> &[Vec<Point<T>>] {
        &self.holes
    }

    /// The polygon's bounding box in storage space (degrees for `f64`
    /// polygons, scaled integers for [`I32Polygon`]).
    pub fn rect(&self) -> Rect {
        Rect {
            min: Point {
                x: self.min.x.to_f64(),
                y: self.min.y.to_f64(),
            },
            max: Point {
                x: self.max.x.to_f64(),
                y: self.max.y.to_f64(),
            },
        }
    }

    /// The factor a degree coordinate is multiplied by to reach storage
    /// space: `1.0` for `f64` polygons, e.g. `1e5` for [`I32Polygon`].
    pub fn scale(&self) -> f64 {
        self.scale
    }

    pub fn options(&self) -> PolygonBuildOptions {
        self.options
    }

    pub fn index_stats(&self) -> &PolygonIndexStats {
        &self.index_stats
    }

    pub fn set_exterior(&mut self, exterior: Vec<Point<T>>) {
        self.exterior = exterior;
        self.rebuild_cache();
    }

    pub fn set_holes(&mut self, holes: Vec<Vec<Point<T>>>) {
        self.holes = holes;
        self.rebuild_cache();
    }

    pub fn set_options(&mut self, options: PolygonBuildOptions) {
        self.options = options;
        self.rebuild_cache();
    }
}

impl Polygon {
    /// Create a new Polygon instance from exterior and holes.
    ///
    /// Example:
    ///
    /// ```rust
    /// use std::vec;
    /// use geometry_rs;
    /// let poly = geometry_rs::Polygon::new(
    ///     vec![
    ///         geometry_rs::Point {
    ///             x: 90.48826291293898,
    ///             y: 45.951129815858565,
    ///         },
    ///         geometry_rs::Point {
    ///             x: 90.48826291293898,
    ///             y: 27.99437617512571,
    ///         },
    ///         geometry_rs::Point {
    ///             x: 122.83201291294,
    ///             y: 27.99437617512571,
    ///         },
    ///         geometry_rs::Point {
    ///             x: 122.83201291294,
    ///             y: 45.951129815858565,
    ///         },
    ///         geometry_rs::Point {
    ///             x: 90.48826291293898,
    ///             y: 45.951129815858565,
    ///         },
    ///     ],
    ///     vec![],
    ///     None,
    /// );
    ///
    /// let p_out = geometry_rs::Point {
    ///     x: 130.74216916294148,
    ///     y: 37.649011392900306,
    /// };
    ///
    /// print!("{:?}\n", poly.contains_point(p_out));
    ///
    /// let p_in = geometry_rs::Point {
    ///     x: 99.9804504129416,
    ///     y: 39.70716466970461,
    /// };
    /// print!("{:?}\n", poly.contains_point(p_in));
    /// ```
    pub fn new(
        exterior: Vec<Point>,
        holes: Vec<Vec<Point>>,
        options: Option<PolygonBuildOptions>,
    ) -> Polygon {
        Self::build(exterior, holes, 1.0, I32RaycastMode::Float, options)
    }
}

impl I32Polygon {
    /// Create a polygon from scaled integer coordinates with no acceleration
    /// index.
    ///
    /// (There is intentionally no `I32Polygon::new`: a second inherent `new`
    /// would make plain `Polygon::new(...)` calls ambiguous.)
    pub fn new_with_mode(
        exterior: Vec<I32Point>,
        holes: Vec<Vec<I32Point>>,
        scale: f64,
        raycast_mode: I32RaycastMode,
    ) -> Self {
        Self::new_with_options(exterior, holes, scale, raycast_mode, None)
    }

    /// Create a polygon from scaled integer coordinates with explicit build
    /// options; the acceleration indexes operate directly in the scaled
    /// integer storage space. The indexes only serve the default (float)
    /// raycast; [`I32RaycastMode::Integer`] always scans segments linearly.
    pub fn new_with_options(
        exterior: Vec<I32Point>,
        holes: Vec<Vec<I32Point>>,
        scale: f64,
        raycast_mode: I32RaycastMode,
        options: Option<PolygonBuildOptions>,
    ) -> Self {
        assert!(!exterior.is_empty(), "polygon exterior must not be empty");
        assert!(
            scale.is_finite() && scale > 0.0,
            "scale must be positive and finite"
        );
        Self::build(exterior, holes, scale, raycast_mode, options)
    }
}

impl<T: CoordStorage> ContainsPoint<Point> for Polygon<T> {
    fn contains_point(&self, point: Point) -> bool {
        Polygon::contains_point(self, point)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Segment {
    pub a: Point,
    pub b: Point,
}

impl Segment {
    pub fn rect(&self) -> Rect {
        let mut min_x: f64 = self.a.x;
        let mut min_y: f64 = self.a.y;
        let mut max_x: f64 = self.b.x;
        let mut max_y: f64 = self.b.y;

        if min_x > max_x {
            let actual_min_x = max_x;
            let actual_max_x = min_x;
            min_x = actual_min_x;
            max_x = actual_max_x;
        }

        if min_y > max_y {
            let actual_min_y = max_y;
            let actual_max_y = min_y;
            min_y = actual_min_y;
            max_y = actual_max_y;
        }

        return Rect {
            min: Point { x: min_x, y: min_y },
            max: Point { x: max_x, y: max_y },
        };
    }
}

pub struct RaycastResult {
    inside: bool, // point on the left
    on: bool,     // point is directly on top of
}

pub fn raycast(seg: &Segment, point: Point) -> RaycastResult {
    let mut p = point;
    let a = seg.a;
    let b = seg.b;

    // make sure that the point is inside the segment bounds
    if a.y < b.y && (p.y < a.y || p.y > b.y) {
        return RaycastResult {
            inside: false,
            on: false,
        };
    } else if a.y > b.y && (p.y < b.y || p.y > a.y) {
        return RaycastResult {
            inside: false,
            on: false,
        };
    }

    // test if point is in on the segment
    if a.y == b.y {
        if a.x == b.x {
            if p.x == a.x && p.y == a.y {
                return RaycastResult {
                    inside: false,
                    on: true,
                };
            }
            return RaycastResult {
                inside: false,
                on: false,
            };
        }
        if p.y == b.y {
            // horizontal segment
            // check if the point in on the line
            if a.x < b.x {
                if p.x >= a.x && p.x <= b.x {
                    return RaycastResult {
                        inside: false,
                        on: true,
                    };
                }
            } else if p.x >= b.x && p.x <= a.x {
                return RaycastResult {
                    inside: false,
                    on: true,
                };
            }
        }
    }
    if a.x == b.x && p.x == b.x {
        // vertical segment
        // check if the point in on the line
        if a.y < b.y {
            if p.y >= a.y && p.y <= b.y {
                return RaycastResult {
                    inside: false,
                    on: true,
                };
            }
        } else if p.y >= b.y && p.y <= a.y {
            return RaycastResult {
                inside: false,
                on: true,
            };
        }
    }
    if (p.x - a.x) / (b.x - a.x) == (p.y - a.y) / (b.y - a.y) {
        return RaycastResult {
            inside: false,
            on: true,
        };
    }

    // do the actual raycast here.
    while p.y == a.y || p.y == b.y {
        p.y = p.y.next_after(std::f64::INFINITY);
    }

    if a.y < b.y {
        if p.y < a.y || p.y > b.y {
            return RaycastResult {
                inside: false,
                on: false,
            };
        }
    } else if p.y < b.y || p.y > a.y {
        return RaycastResult {
            inside: false,
            on: false,
        };
    }
    if a.x > b.x {
        if p.x >= a.x {
            return RaycastResult {
                inside: false,
                on: false,
            };
        }
        if p.x <= b.x {
            return RaycastResult {
                inside: true,
                on: false,
            };
        }
    } else {
        if p.x >= b.x {
            return RaycastResult {
                inside: false,
                on: false,
            };
        }
        if p.x <= a.x {
            return RaycastResult {
                inside: true,
                on: false,
            };
        }
    }
    if a.y < b.y {
        if (p.y - a.y) / (p.x - a.x) >= (b.y - a.y) / (b.x - a.x) {
            return RaycastResult {
                inside: true,
                on: false,
            };
        }
    } else if (p.y - b.y) / (p.x - b.x) >= (a.y - b.y) / (a.x - b.x) {
        return RaycastResult {
            inside: true,
            on: false,
        };
    }
    return RaycastResult {
        inside: false,
        on: false,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    fn square(min: f64, max: f64) -> Vec<Point> {
        vec![
            Point { x: min, y: min },
            Point { x: min, y: max },
            Point { x: max, y: max },
            Point { x: max, y: min },
            Point { x: min, y: min },
        ]
    }

    fn polygon_with_segments(segments: usize) -> Vec<Point> {
        let mut ring = Vec::with_capacity(segments + 1);
        for i in 0..segments {
            let theta = (i as f64) / (segments as f64) * std::f64::consts::TAU;
            ring.push(Point {
                x: theta.cos(),
                y: theta.sin(),
            });
        }
        ring.push(ring[0]);
        ring
    }

    fn scan_candidates(ring: &[Point], y: f64, x_min: f64, x_max: f64) -> Vec<usize> {
        let mut out = Vec::new();
        let query = Rect {
            min: Point { x: x_min, y },
            max: Point { x: x_max, y },
        };
        for (i, pair) in ring.windows(2).enumerate() {
            let seg = Segment {
                a: pair[0],
                b: pair[1],
            };
            if seg.rect().intersects_rect(query) {
                out.push(i);
            }
        }
        out
    }

    #[test]
    fn rings_contains_point_allow_on_edge() {
        let ring = square(0.0, 10.0);
        let on_edge = Point { x: 0.0, y: 5.0 };
        assert!(rings_contains_point(&ring, None, on_edge, true));
        assert!(!rings_contains_point(&ring, None, on_edge, false));
    }

    #[test]
    fn polygon_contains_basic_in_and_out() {
        let poly = Polygon::new(square(0.0, 10.0), vec![], None);
        assert!(poly.contains_point(Point { x: 5.0, y: 5.0 }));
        assert!(!poly.contains_point(Point { x: 20.0, y: 5.0 }));
    }

    #[test]
    fn polygon_contains_with_hole() {
        let poly = Polygon::new(square(0.0, 10.0), vec![square(3.0, 7.0)], None);
        assert!(poly.contains_point(Point { x: 1.0, y: 1.0 }));
        assert!(!poly.contains_point(Point { x: 5.0, y: 5.0 }));
    }

    #[test]
    fn i32_polygon_contains_with_hole_and_fractional_query() {
        let square = |min, max| {
            vec![
                I32Point { x: min, y: min },
                I32Point { x: min, y: max },
                I32Point { x: max, y: max },
                I32Point { x: max, y: min },
                I32Point { x: min, y: min },
            ]
        };
        for mode in [I32RaycastMode::Float, I32RaycastMode::Integer] {
            let poly = I32Polygon::new_with_mode(
                square(0, 1_000_000),
                vec![square(400_000, 600_000)],
                1e5,
                mode,
            );
            assert!(poly.contains_point(Point {
                x: 1.23456,
                y: 2.34567
            }));
            assert!(!poly.contains_point(Point { x: 5.0, y: 5.0 }));
            assert!(!poly.contains_point(Point { x: 11.0, y: 5.0 }));
        }
    }

    #[test]
    fn indexed_and_non_indexed_results_match() {
        let ring = polygon_with_segments(128);
        let p_in = Point { x: 0.2, y: 0.1 };
        let p_out = Point { x: 2.0, y: 0.0 };

        let p1 = Polygon::new(
            ring.clone(),
            vec![],
            Some(PolygonBuildOptions {
                enable_rtree: false,
                enable_compressed_quad: false,
                enable_y_stripes: false,
                rtree_min_segments: 64,
            }),
        );

        let p2 = Polygon::new(
            ring,
            vec![],
            Some(PolygonBuildOptions {
                enable_rtree: true,
                enable_compressed_quad: true,
                enable_y_stripes: false,
                rtree_min_segments: 64,
            }),
        );

        assert_eq!(p1.contains_point(p_in), p2.contains_point(p_in));
        assert_eq!(p1.contains_point(p_out), p2.contains_point(p_out));
    }

    #[test]
    fn rtree_and_compressed_quad_and_both_match_baseline() {
        let ring = polygon_with_segments(160);
        let points = [
            Point { x: 0.2, y: 0.1 },
            Point { x: -0.4, y: -0.3 },
            Point { x: 1.2, y: 0.0 },
        ];

        let base = Polygon::new(
            ring.clone(),
            vec![],
            Some(PolygonBuildOptions {
                enable_rtree: false,
                enable_compressed_quad: false,
                enable_y_stripes: false,
                rtree_min_segments: 64,
            }),
        );
        let only_rtree = Polygon::new(
            ring.clone(),
            vec![],
            Some(PolygonBuildOptions {
                enable_rtree: true,
                enable_compressed_quad: false,
                enable_y_stripes: false,
                rtree_min_segments: 64,
            }),
        );
        let only_compressed = Polygon::new(
            ring.clone(),
            vec![],
            Some(PolygonBuildOptions {
                enable_rtree: false,
                enable_compressed_quad: true,
                enable_y_stripes: false,
                rtree_min_segments: 64,
            }),
        );
        let both = Polygon::new(
            ring,
            vec![],
            Some(PolygonBuildOptions {
                enable_rtree: true,
                enable_compressed_quad: true,
                enable_y_stripes: false,
                rtree_min_segments: 64,
            }),
        );

        for p in points {
            let expected = base.contains_point(p);
            assert_eq!(only_rtree.contains_point(p), expected);
            assert_eq!(only_compressed.contains_point(p), expected);
            assert_eq!(both.contains_point(p), expected);
        }
    }

    #[test]
    fn threshold_boundaries_63_64_65_are_consistent() {
        let ring = polygon_with_segments(65);
        let p_in = Point { x: 0.2, y: 0.0 };
        let p_out = Point { x: 1.5, y: 0.0 };

        let p63 = Polygon::new(
            ring.clone(),
            vec![],
            Some(PolygonBuildOptions {
                enable_rtree: true,
                enable_compressed_quad: true,
                enable_y_stripes: false,
                rtree_min_segments: 63,
            }),
        );
        let p64 = Polygon::new(
            ring.clone(),
            vec![],
            Some(PolygonBuildOptions {
                enable_rtree: true,
                enable_compressed_quad: true,
                enable_y_stripes: false,
                rtree_min_segments: 64,
            }),
        );
        let p65 = Polygon::new(
            ring,
            vec![],
            Some(PolygonBuildOptions {
                enable_rtree: true,
                enable_compressed_quad: true,
                enable_y_stripes: false,
                rtree_min_segments: 65,
            }),
        );

        assert_eq!(p63.contains_point(p_in), p64.contains_point(p_in));
        assert_eq!(p64.contains_point(p_in), p65.contains_point(p_in));

        assert_eq!(p63.contains_point(p_out), p64.contains_point(p_out));
        assert_eq!(p64.contains_point(p_out), p65.contains_point(p_out));
    }

    #[test]
    fn setters_rebuild_cache_and_keep_correct_results() {
        let mut poly = Polygon::new(square(0.0, 10.0), vec![], None);
        assert!(poly.contains_point(Point { x: 1.0, y: 1.0 }));

        poly.set_exterior(square(20.0, 30.0));
        assert!(!poly.contains_point(Point { x: 1.0, y: 1.0 }));
        assert!(poly.contains_point(Point { x: 21.0, y: 21.0 }));

        poly.set_holes(vec![square(22.0, 24.0)]);
        assert!(!poly.contains_point(Point { x: 23.0, y: 23.0 }));

        poly.set_options(PolygonBuildOptions {
            enable_rtree: false,
            enable_compressed_quad: false,
            enable_y_stripes: false,
            rtree_min_segments: 64,
        });
        assert!(!poly.contains_point(Point { x: 23.0, y: 23.0 }));
        assert!(poly.contains_point(Point { x: 25.0, y: 25.0 }));
    }

    #[test]
    fn compressed_quad_candidates_match_scan() {
        let ring = polygon_with_segments(256);
        let options = PolygonBuildOptions {
            enable_rtree: false,
            enable_compressed_quad: true,
            enable_y_stripes: false,
            rtree_min_segments: 64,
        };
        let (index, stats) = build_ring_index(&ring, &options);
        let index = index.unwrap();

        assert!(stats.used_compressed_quad);
        assert!(!stats.below_threshold);

        for y in [-1.2, -1.0, -0.75, -0.2, 0.0, 0.4, 0.99, 1.0, 1.2] {
            let mut from_index = Vec::new();
            index.search_candidates(&ring, y, &mut from_index);
            from_index.sort_unstable();

            let mut from_scan = scan_candidates(&ring, y, index.x_min, index.x_max);
            from_scan.sort_unstable();

            assert_eq!(from_index, from_scan);
        }
    }

    #[test]
    fn y_stripes_candidates_match_scan() {
        let ring = polygon_with_segments(256);
        let options = PolygonBuildOptions {
            enable_rtree: false,
            enable_compressed_quad: false,
            enable_y_stripes: true,
            rtree_min_segments: 64,
        };
        let (index, stats) = build_ring_index(&ring, &options);
        let index = index.unwrap();

        assert!(stats.used_y_stripes);
        assert!(!stats.below_threshold);
        let y_stats = stats.y_stripes.as_ref().unwrap();
        assert_eq!(y_stats.segment_count, 256);
        assert!(y_stats.stripe_count >= 32);
        assert!(y_stats.assigned_item_count >= 256);
        assert!(y_stats.max_bucket_len > 0);

        for y in [-1.2, -1.0, -0.75, -0.2, 0.0, 0.4, 0.99, 1.0, 1.2] {
            let mut from_index = Vec::new();
            index.search_candidates(&ring, y, &mut from_index);
            from_index.sort_unstable();

            let mut from_scan = scan_candidates(&ring, y, index.x_min, index.x_max);
            from_scan.sort_unstable();

            assert_eq!(from_index, from_scan);
        }
    }

    #[test]
    fn polygon_index_stats_capture_threshold_for_compressed_quad() {
        let indexed = Polygon::new(
            polygon_with_segments(256),
            vec![polygon_with_segments(32)],
            Some(PolygonBuildOptions {
                enable_rtree: false,
                enable_compressed_quad: true,
                enable_y_stripes: false,
                rtree_min_segments: 64,
            }),
        );
        let stats = indexed.index_stats();
        assert!(stats.exterior.used_compressed_quad);
        assert!(!stats.exterior.below_threshold);
        assert_eq!(stats.exterior.segment_count, 256);
        assert_eq!(stats.holes.len(), 1);
        assert!(stats.holes[0].below_threshold);
        assert!(!stats.holes[0].used_compressed_quad);
    }

    fn i32_polygon_with_segments(segments: usize, scale: f64) -> Vec<I32Point> {
        let mut ring = Vec::with_capacity(segments + 1);
        for i in 0..segments {
            let theta = (i as f64) / (segments as f64) * std::f64::consts::TAU;
            ring.push(I32Point {
                x: (theta.cos() * scale).round() as i32,
                y: (theta.sin() * scale).round() as i32,
            });
        }
        ring.push(ring[0]);
        ring
    }

    fn y_stripes_options() -> PolygonBuildOptions {
        PolygonBuildOptions {
            enable_rtree: false,
            enable_compressed_quad: false,
            enable_y_stripes: true,
            rtree_min_segments: 64,
        }
    }

    #[test]
    fn i32_y_stripes_matches_linear_scan() {
        let scale = 1e5;
        let ring = i32_polygon_with_segments(256, scale);
        let hole = i32_polygon_with_segments(96, scale * 0.4);

        let linear = I32Polygon::new_with_mode(
            ring.clone(),
            vec![hole.clone()],
            scale,
            I32RaycastMode::Float,
        );
        let indexed = I32Polygon::new_with_options(
            ring,
            vec![hole],
            scale,
            I32RaycastMode::Float,
            Some(y_stripes_options()),
        );
        assert!(indexed.index_stats().exterior.used_y_stripes);
        assert!(indexed.index_stats().holes[0].used_y_stripes);

        // Sweep radii crossing the hole boundary, the fill ring and the
        // exterior boundary, including points snapped to the 1e-5 grid.
        for i in 0..64 {
            let theta = (i as f64) / 64.0 * std::f64::consts::TAU;
            for r in [0.0, 0.2, 0.39999, 0.4, 0.40001, 0.7, 0.99999, 1.0, 1.1] {
                let p = Point {
                    x: theta.cos() * r,
                    y: theta.sin() * r,
                };
                assert_eq!(
                    linear.contains_point(p),
                    indexed.contains_point(p),
                    "mismatch at {p:?}"
                );
                let snapped = Point {
                    x: (p.x * scale).round() / scale,
                    y: (p.y * scale).round() / scale,
                };
                assert_eq!(
                    linear.contains_point(snapped),
                    indexed.contains_point(snapped),
                    "mismatch at snapped {snapped:?}"
                );
            }
        }
    }

    #[test]
    fn i32_y_stripes_matches_f64_y_stripes_away_from_boundary() {
        let scale = 1e5;
        let int_ring = i32_polygon_with_segments(256, scale);
        let float_ring: Vec<Point> = int_ring
            .iter()
            .map(|p| Point {
                x: f64::from(p.x) / scale,
                y: f64::from(p.y) / scale,
            })
            .collect();

        let int_poly = I32Polygon::new_with_options(
            int_ring,
            vec![],
            scale,
            I32RaycastMode::Float,
            Some(y_stripes_options()),
        );
        let float_poly = Polygon::new(float_ring, vec![], Some(y_stripes_options()));

        for i in 0..256 {
            let theta = (i as f64) / 256.0 * std::f64::consts::TAU;
            for r in [0.0, 0.3, 0.6, 0.9, 1.1, 1.5] {
                let p = Point {
                    x: theta.cos() * r,
                    y: theta.sin() * r,
                };
                assert_eq!(
                    float_poly.contains_point(p),
                    int_poly.contains_point(p),
                    "mismatch at {p:?}"
                );
            }
        }
    }

    #[test]
    fn i32_y_stripes_candidates_match_scan() {
        let scale = 1e5;
        let ring = i32_polygon_with_segments(256, scale);
        let (index, stats) = build_ring_index(&ring, &y_stripes_options());
        let index = index.unwrap();
        assert!(stats.used_y_stripes);

        let float_ring: Vec<Point> = ring
            .iter()
            .map(|p| Point {
                x: f64::from(p.x),
                y: f64::from(p.y),
            })
            .collect();

        for y in [
            -1.2e5, -1e5, -0.75e5, -0.2e5, 0.0, 0.4e5, 0.99e5, 1e5, 1.2e5, 33333.0,
        ] {
            let mut from_index = Vec::new();
            index.search_candidates(&ring, y, &mut from_index);
            from_index.sort_unstable();

            let mut from_scan = scan_candidates(&float_ring, y, index.x_min, index.x_max);
            from_scan.sort_unstable();

            assert_eq!(from_index, from_scan, "candidate mismatch at y={y}");
        }
    }

    #[test]
    fn polygon_index_stats_capture_y_stripes_metrics() {
        let indexed = Polygon::new(
            polygon_with_segments(256),
            vec![polygon_with_segments(32)],
            Some(PolygonBuildOptions {
                enable_rtree: false,
                enable_compressed_quad: false,
                enable_y_stripes: true,
                rtree_min_segments: 64,
            }),
        );
        let stats = indexed.index_stats();
        assert!(stats.exterior.used_y_stripes);
        assert!(!stats.exterior.below_threshold);
        assert_eq!(stats.exterior.segment_count, 256);
        assert!(stats.exterior.y_stripes.is_some());
        assert_eq!(stats.holes.len(), 1);
        assert!(stats.holes[0].below_threshold);
        assert!(!stats.holes[0].used_y_stripes);
        assert!(stats.holes[0].y_stripes.is_none());
    }
}
