# Geometry-rs [![docs.rs](https://img.shields.io/docsrs/geometry-rs)](https://docs.rs/geometry-rs/latest/geometry_rs/)

Rewrite parts of [tidwall/geometry](https://github.com/tidwall/geometry) to Rust
for [ringsaturn/tzf-rs](https://github.com/ringsaturn/tzf-rs).

```toml
[dependencies]
geometry-rs = "0.1.2"
```

```rust
use std::vec;

use geometry_rs;

fn main() {
    let poly = geometry_rs::Polygon::new(
        vec![
            geometry_rs::Point {
                x: 90.48826291293898,
                y: 45.951129815858565,
            },
            geometry_rs::Point {
                x: 90.48826291293898,
                y: 27.99437617512571,
            },
            geometry_rs::Point {
                x: 122.83201291294,
                y: 27.99437617512571,
            },
            geometry_rs::Point {
                x: 122.83201291294,
                y: 45.951129815858565,
            },
            geometry_rs::Point {
                x: 90.48826291293898,
                y: 45.951129815858565,
            },
        ],
        vec![],
    );

    let p_out = geometry_rs::Point {
        x: 130.74216916294148,
        y: 37.649011392900306,
    };

    print!("{:?}\n", poly.contains_point(p_out));

    let p_in = geometry_rs::Point {
        x: 99.9804504129416,
        y: 39.70716466970461,
    };
    print!("{:?}\n", poly.contains_point(p_in));
}
```
