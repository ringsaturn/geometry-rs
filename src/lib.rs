//! Rewrite some features of [tidwall/geometry](https://github.com/tidwall/geometry) to Rust
//! for [ringsaturn/tzf-rs](https://github.com/ringsaturn/tzf-rs).

#![allow(unused_imports)]
#![allow(dead_code)]

use float_next_after::NextAfter;
use rtree_rs::RTree;
use rtree_rs::Rect as RTreeRect;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct Rect {
    min: Point,
    max: Point,
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
            0 => return self.south(),
            1 => return self.east(),
            2 => return self.north(),
            3 => return self.west(),
            _ => return self.south(), // TODO(ringsaturn): raise err
        }
    }
}

fn segment_at_for_vec_point(exterior: &Vec<Point>, index: i64) -> Segment {
    let seg_a: Point = exterior[index as usize];
    let mut seg_b_index = index;
    if seg_b_index == (exterior.len() - 1) as i64 {
        seg_b_index = 0
    } else {
        seg_b_index += 1
    }
    let seg_b: Point = exterior[seg_b_index as usize];
    return Segment { a: seg_a, b: seg_b };
}

fn rings_contains_point(ring: &Vec<Point>, point: Point, allow_on_edge: bool) -> bool {
    let rect = Rect {
        min: Point {
            x: std::f64::NEG_INFINITY,
            y: point.y,
        },
        max: Point {
            x: std::f64::INFINITY,
            y: point.y,
        },
    };
    let mut inside: bool = false;
    let n: i64 = (ring.len() - 1) as i64;
    for i in 0..n {
        let seg: Segment = segment_at_for_vec_point(&ring, i);

        if seg.rect().intersects_rect(rect) {
            let res: RaycastResult = raycast(&seg, point);
            // print!("res= inside:{:?} on:{:?}\n", res.inside, res.on);
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

fn rings_contains_point_by_rtree_index(
    ring: &Vec<Point>,
    ring_rtree: &rtree_rs::RTree<2, f64, i64>,
    point: Point,
    allow_on_edge: bool,
) -> bool {
    let rect = Rect {
        min: Point {
            x: std::f64::NEG_INFINITY,
            y: point.y,
        },
        max: Point {
            x: std::f64::INFINITY,
            y: point.y,
        },
    };
    for item in ring_rtree.search(RTreeRect::new(
        [std::f64::NEG_INFINITY, point.y],
        [std::f64::INFINITY, point.y],
    )) {
        let seg: Segment = segment_at_for_vec_point(&ring, *item.data);
        let irect = seg.rect();
        if irect.intersects_rect(rect) {
            let res: RaycastResult = raycast(&seg, point);
            if res.on {
                return allow_on_edge;
            }
            if res.inside {
                return true;
            }
        }
    }
    return false;
}

// #[derive(Clone, Debug)]
pub struct Polygon {
    exterior: Vec<Point>,
    exterior_rtree: rtree_rs::RTree<2, f64, i64>,
    holes: Vec<Vec<Point>>,
    holes_rtree: Vec<rtree_rs::RTree<2, f64, i64>>,
    rect: Rect,
    with_index: bool,
}

impl Polygon {
    /// Point-In-Polygon check with RTree index.
    /// `with_index` param must true for this query.
    /// Query speed is faster compare with [contains_point_normal].
    fn contains_point_with_index(&self, p: Point) -> bool {
        if !rings_contains_point_by_rtree_index(&self.exterior, &self.exterior_rtree, p, false) {
            return false;
        }

        let mut contains: bool = true;
        let mut i: usize = 0;
        for hole in self.holes.iter() {
            let tr = self.holes_rtree.get(i).unwrap();
            if rings_contains_point_by_rtree_index(&hole, &tr, p, false) {
                contains = false;
                break;
            }

            i += 1;
        }
        return contains;
    }

    /// Point-In-Polygon check, the normal way.
    /// It's most used algorithm implementation, port from Go's [geojson]
    ///
    /// [geojson]: https://github.com/tidwall/geojson
    fn contains_point_normal(&self, p: Point) -> bool {
        if !rings_contains_point(&self.exterior, p, false) {
            return false;
        }
        let mut contains: bool = true;
        for hole in self.holes.iter() {
            if rings_contains_point(&hole, p, false) {
                contains = false;
                break;
            }
        }
        return contains;
    }

    /// Do point-in-polygon search.
    ///
    /// NOTE: `with_index` could increase performance, but requires more memory.
    pub fn contains_point(&self, p: Point) -> bool {
        if !self.rect.contains_point(p) {
            return false;
        }
        if self.with_index {
            return self.contains_point_with_index(p);
        }
        return self.contains_point_normal(p);
    }

    pub fn new(exterior: Vec<Point>, holes: Vec<Vec<Point>>, with_index: bool) -> Polygon {
        let mut minx: f64 = exterior.get(0).unwrap().x;
        let mut miny: f64 = exterior.get(0).unwrap().y;
        let mut maxx: f64 = exterior.get(0).unwrap().x;
        let mut maxy: f64 = exterior.get(0).unwrap().y;

        // for p in exterior.iter() {
        for i in 0..exterior.len() - 1 {
            let p = exterior[i];
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

        let rect = Rect {
            min: Point { x: minx, y: miny },
            max: Point { x: maxx, y: maxy },
        };

        let mut exterior_rtree = RTree::new();
        let n = (exterior.len() - 1) as i64;
        for i in 0..n {
            let segrect = segment_at_for_vec_point(&exterior, i).rect();
            if with_index {
                exterior_rtree.insert(
                    RTreeRect::new(
                        [segrect.min.x, segrect.min.y],
                        [segrect.max.x, segrect.max.y],
                    ),
                    i as i64,
                );
            }
        }

        let mut holes_rtree = vec![];
        for hole_poly in holes.iter() {
            let mut hole_rtre = RTree::new();
            let n = (hole_poly.len() - 1) as i64;
            for i in 0..n {
                let segrect = segment_at_for_vec_point(&hole_poly, i).rect();
                if with_index {
                    hole_rtre.insert(
                        RTreeRect::new(
                            [segrect.min.x, segrect.min.y],
                            [segrect.max.x, segrect.max.y],
                        ),
                        i as i64,
                    );
                }
            }
            if with_index {
                holes_rtree.push(hole_rtre);
            }
        }

        return Polygon {
            exterior,
            exterior_rtree,
            holes,
            holes_rtree,
            rect,
            with_index,
        };
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Segment {
    a: Point,
    b: Point,
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
            } else {
                if p.x >= b.x && p.x <= a.x {
                    return RaycastResult {
                        inside: false,
                        on: true,
                    };
                }
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
        } else {
            if p.y >= b.y && p.y <= a.y {
                return RaycastResult {
                    inside: false,
                    on: true,
                };
            }
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
        // p.y = NextAfter(p.y, &std::f64::INFINITY)
        // let next = big_num.next_after(&std::f64::INFINITY);
        p.y = p.y.next_after(std::f64::INFINITY);
    }

    if a.y < b.y {
        if p.y < a.y || p.y > b.y {
            return RaycastResult {
                inside: false,
                on: false,
            };
        }
    } else {
        if p.y < b.y || p.y > a.y {
            return RaycastResult {
                inside: false,
                on: false,
            };
        }
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
    } else {
        if (p.y - b.y) / (p.x - b.x) >= (a.y - b.y) / (a.x - b.x) {
            return RaycastResult {
                inside: true,
                on: false,
            };
        }
    }
    return RaycastResult {
        inside: false,
        on: false,
    };
}
