#![feature(test)]
#[cfg(test)]

mod benches_polygon {
    use geometry_rs;
    extern crate test;
    use test::Bencher;

    fn load_polygon() -> geometry_rs::Polygon {
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
        return poly;
    }

    #[bench]
    fn poly_contain_point(b: &mut Bencher) {
        let poly = load_polygon();

        let p_in = geometry_rs::Point {
            x: 99.9804504129416,
            y: 39.70716466970461,
        };

        b.iter(|| {
            let _ = poly.contains_point(p_in);
        });
    }
}
