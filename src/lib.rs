#![doc = include_str!("../README.md")]

mod next_after;

use crate::next_after::NextAfter;
use rtree_rs::{RTree, Rect as RTreeRect};

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
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
    pub rtree_min_segments: usize,
}

impl Default for PolygonBuildOptions {
    fn default() -> Self {
        Self {
            enable_rtree: true,
            enable_compressed_quad: true,
            rtree_min_segments: 64,
        }
    }
}

struct RingIndex {
    x_min: f64,
    x_max: f64,
    seg_count: usize,
    rtree: Option<RTree<2, f64, usize>>,
    compressed_quad: Option<CompressedQuadIndex>,
}

impl RingIndex {
    fn search_candidates(&self, point_y: f64, out: &mut Vec<usize>) {
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
    fn build(ring: &[Point]) -> Option<Self> {
        let seg_count = ring_segment_count(ring);
        if seg_count == 0 {
            return None;
        }

        let mut min_x = ring[0].x;
        let mut min_y = ring[0].y;
        let mut max_x = ring[0].x;
        let mut max_y = ring[0].y;

        for p in ring.iter().take(seg_count) {
            if p.x < min_x {
                min_x = p.x;
            }
            if p.y < min_y {
                min_y = p.y;
            }
            if p.x > max_x {
                max_x = p.x;
            }
            if p.y > max_y {
                max_y = p.y;
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

fn ring_segment_count(ring: &[Point]) -> usize {
    ring.len().saturating_sub(1)
}

fn segment_at_for_slice(ring: &[Point], index: usize) -> Segment {
    Segment {
        a: ring[index],
        b: ring[index + 1],
    }
}

fn build_ring_index(ring: &[Point], options: &PolygonBuildOptions) -> Option<RingIndex> {
    if !options.enable_rtree && !options.enable_compressed_quad {
        return None;
    }
    if ring.is_empty() {
        return None;
    }

    let seg_count = ring_segment_count(ring);
    if seg_count < options.rtree_min_segments {
        return None;
    }

    let mut x_min = ring[0].x;
    let mut x_max = ring[0].x;
    for p in ring.iter().take(seg_count) {
        if p.x < x_min {
            x_min = p.x;
        }
        if p.x > x_max {
            x_max = p.x;
        }
    }

    let rtree = if options.enable_rtree {
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
        CompressedQuadIndex::build(ring)
    } else {
        None
    };

    if rtree.is_none() && compressed_quad.is_none() {
        return None;
    }

    Some(RingIndex {
        x_min,
        x_max,
        seg_count,
        rtree,
        compressed_quad,
    })
}

fn rings_contains_point(
    ring: &[Point],
    ring_index: Option<&RingIndex>,
    point: Point,
    allow_on_edge: bool,
) -> bool {
    let mut inside: bool = false;

    if let Some(index) = ring_index {
        let mut candidates = Vec::new();
        index.search_candidates(point.y, &mut candidates);
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

    let ray_rect = Rect {
        min: Point {
            x: std::f64::NEG_INFINITY,
            y: point.y,
        },
        max: Point {
            x: std::f64::INFINITY,
            y: point.y,
        },
    };

    for pair in ring.windows(2) {
        let seg = Segment {
            a: pair[0],
            b: pair[1],
        };

        if seg.rect().intersects_rect(ray_rect) {
            let res: RaycastResult = raycast(&seg, point);
            if res.on {
                inside = allow_on_edge;
                break;
            }
            if res.inside {
                inside = !inside;
            }
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

pub struct Polygon {
    exterior: Vec<Point>,
    holes: Vec<Vec<Point>>,
    rect: Rect,
    options: PolygonBuildOptions,
    exterior_index: Option<RingIndex>,
    hole_indexes: Vec<Option<RingIndex>>,
}

impl Polygon {
    fn compute_rect(exterior: &[Point]) -> Rect {
        let mut minx: f64 = exterior[0].x;
        let mut miny: f64 = exterior[0].y;
        let mut maxx: f64 = exterior[0].x;
        let mut maxy: f64 = exterior[0].y;

        for p in exterior.iter() {
            if p.x < minx {
                minx = p.x;
            }
            if p.y < miny {
                miny = p.y;
            }
            if p.x > maxx {
                maxx = p.x;
            }
            if p.y > maxy {
                maxy = p.y;
            }
        }

        Rect {
            min: Point { x: minx, y: miny },
            max: Point { x: maxx, y: maxy },
        }
    }

    fn rebuild_cache(&mut self) {
        self.rect = Self::compute_rect(&self.exterior);
        self.exterior_index = build_ring_index(&self.exterior, &self.options);

        self.hole_indexes = self
            .holes
            .iter()
            .map(|hole| build_ring_index(hole, &self.options))
            .collect();
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

    /// Do point-in-polygon search.
    pub fn contains_point(&self, p: Point) -> bool {
        if !self.rect.contains_point(p) {
            return false;
        }

        return self.contains_point_normal(p);
    }

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
        let mut poly = Polygon {
            exterior,
            holes,
            rect: Rect {
                min: Point { x: 0.0, y: 0.0 },
                max: Point { x: 0.0, y: 0.0 },
            },
            options: options.unwrap_or_default(),
            exterior_index: None,
            hole_indexes: Vec::new(),
        };
        poly.rebuild_cache();
        poly
    }

    pub fn exterior(&self) -> &[Point] {
        &self.exterior
    }

    pub fn holes(&self) -> &[Vec<Point>] {
        &self.holes
    }

    pub fn rect(&self) -> Rect {
        self.rect
    }

    pub fn options(&self) -> PolygonBuildOptions {
        self.options
    }

    pub fn set_exterior(&mut self, exterior: Vec<Point>) {
        self.exterior = exterior;
        self.rebuild_cache();
    }

    pub fn set_holes(&mut self, holes: Vec<Vec<Point>>) {
        self.holes = holes;
        self.rebuild_cache();
    }

    pub fn set_options(&mut self, options: PolygonBuildOptions) {
        self.options = options;
        self.rebuild_cache();
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
                rtree_min_segments: 64,
            }),
        );

        let p2 = Polygon::new(
            ring,
            vec![],
            Some(PolygonBuildOptions {
                enable_rtree: true,
                enable_compressed_quad: true,
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
                rtree_min_segments: 64,
            }),
        );
        let only_rtree = Polygon::new(
            ring.clone(),
            vec![],
            Some(PolygonBuildOptions {
                enable_rtree: true,
                enable_compressed_quad: false,
                rtree_min_segments: 64,
            }),
        );
        let only_compressed = Polygon::new(
            ring.clone(),
            vec![],
            Some(PolygonBuildOptions {
                enable_rtree: false,
                enable_compressed_quad: true,
                rtree_min_segments: 64,
            }),
        );
        let both = Polygon::new(
            ring,
            vec![],
            Some(PolygonBuildOptions {
                enable_rtree: true,
                enable_compressed_quad: true,
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
                rtree_min_segments: 63,
            }),
        );
        let p64 = Polygon::new(
            ring.clone(),
            vec![],
            Some(PolygonBuildOptions {
                enable_rtree: true,
                enable_compressed_quad: true,
                rtree_min_segments: 64,
            }),
        );
        let p65 = Polygon::new(
            ring,
            vec![],
            Some(PolygonBuildOptions {
                enable_rtree: true,
                enable_compressed_quad: true,
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
            rtree_min_segments: 64,
        };
        let index = build_ring_index(&ring, &options).unwrap();

        for y in [-1.2, -1.0, -0.75, -0.2, 0.0, 0.4, 0.99, 1.0, 1.2] {
            let mut from_index = Vec::new();
            index.search_candidates(y, &mut from_index);
            from_index.sort_unstable();

            let mut from_scan = scan_candidates(&ring, y, index.x_min, index.x_max);
            from_scan.sort_unstable();

            assert_eq!(from_index, from_scan);
        }
    }
}
