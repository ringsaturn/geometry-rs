#![feature(test)]
#[cfg(test)]
mod benches_tx_polygon {
    extern crate test;
    use geo::Contains;
    use serde_derive::Deserialize;
    use serde_derive::Serialize;
    use test::Bencher;

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct Feature {
        #[serde(rename = "type")]
        pub type_field: String,
        pub coordinates: Vec<Vec<Vec<Vec<f64>>>>,
    }

    // Test data copy from https://github.com/unitedstates/districts/blob/gh-pages/states/AZ/shape.geojson
    fn load_json_data() -> Vec<u8> {
        include_bytes!("./tx.geojson").to_vec()
    }

    fn load_poly(rtree_flag: bool) -> geometry_rs::Polygon {
        let feature: Feature = serde_json::from_slice(&load_json_data()[..]).unwrap();
        let mut exterior: Vec<geometry_rs::Point> = vec![];
        let interiors: Vec<Vec<geometry_rs::Point>> = vec![];
        for poly in feature.coordinates.iter() {
            for line in poly.iter() {
                for point in line.iter() {
                    exterior.push(geometry_rs::Point {
                        x: point[0],
                        y: point[1],
                    });
                }
            }
        }
        let poly = geometry_rs::Polygon::new_with_rtree_index_opt(exterior, interiors, rtree_flag);
        poly
    }

    #[bench]
    fn poly_contain_point(b: &mut Bencher) {
        let poly = load_poly(false);
        let p_in = geometry_rs::Point { x: -98.52539, y: 29.363027 };

        b.iter(|| {
            let _ = poly.contains_point(p_in);
        });
    }

    #[bench]
    fn poly_not_contain_point(b: &mut Bencher) {
        let poly = load_poly(false);
        let p_in = geometry_rs::Point {
            x: -101.953125,
            y: 29.324720161,
        };

        b.iter(|| {
            let _ = poly.contains_point(p_in);
        });
    }

    #[bench]
    fn poly_with_rtree_contain_point(b: &mut Bencher) {
        let poly = load_poly(true);
        let p_in = geometry_rs::Point { x: -98.52539, y: 29.363027 };

        b.iter(|| {
            let _ = poly.contains_point(p_in);
        });
    }

    #[bench]
    fn poly_with_rtree_not_contain_point(b: &mut Bencher) {
        let poly = load_poly(true);
        let p_in = geometry_rs::Point {
            x: -101.953125,
            y: 29.324720161,
        };

        b.iter(|| {
            let _ = poly.contains_point(p_in);
        });
    }

    fn load_geo_poly() -> geo::Polygon<f64> {
        let feature: Feature = serde_json::from_slice(&load_json_data()[..]).unwrap();
        let mut exterior: Vec<geo::Coord> = vec![];
        let interior: Vec<geo::LineString> = vec![];
        for poly in feature.coordinates.iter() {
            for line in poly.iter() {
                for point in line.iter() {
                    exterior.push(geo::Coord {
                        x: point[0],
                        y: point[1],
                    });
                }
            }
        }
        let poly = geo::Polygon::new(geo::LineString(exterior), interior);
        poly
    }

    #[bench]
    fn geo_poly_contain_point(b: &mut Bencher) {
        let poly = load_geo_poly();
        let p_in = geo::Point(geo::Coord { x: -98.52539, y: 29.363027 });

        b.iter(|| {
            let _ = poly.contains(&p_in);
        });
    }

    #[bench]
    fn geo_poly_not_contain_point(b: &mut Bencher) {
        let poly = load_geo_poly();
        let p_in = geo::Point(geo::Coord {
            x: -101.953125,
            y: 29.324720161,
        });

        b.iter(|| {
            let _ = poly.contains(&p_in);
        });
    }
}
