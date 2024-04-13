#![feature(test)]
#[cfg(test)]

mod benches_point_in_polygon {
    extern crate test;
    use geo::Contains;
    use geometry_rs::{Point, Polygon};
    use serde_derive::Deserialize;
    use serde_derive::Serialize;
    use std::str;
    use test::Bencher;

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Geometry {
        #[serde(rename = "type")]
        type_field: String,
        coordinates: Vec<Vec<Vec<Vec<f64>>>>,
    }

    // Test data copy from https://github.com/unitedstates/districts/blob/gh-pages/states/AZ/shape.geojson
    pub fn load_az_file() -> Vec<u8> {
        include_bytes!("./az.geojson").to_vec()
    }

    // Test data copy from https://github.com/unitedstates/districts/blob/gh-pages/states/TX/shape.geojson
    fn load_tx_file() -> Vec<u8> {
        include_bytes!("./tx.geojson").to_vec()
    }

    fn load_poly(data: Vec<u8>, rtree_index: bool) -> Polygon {
        let geometry: Geometry = serde_json::from_slice(&data).unwrap();

        let mut exterior: Vec<Point> = vec![];
        let mut interior: Vec<Vec<Point>> = vec![];

        for coordinates in geometry.coordinates {
            for (i, v) in coordinates.iter().enumerate() {
                if i == 0 {
                    for point in v {
                        exterior.push(Point {
                            x: point[0],
                            y: point[1],
                        });
                    }
                } else {
                    let mut points = Vec::new();
                    for point in v {
                        points.push(Point {
                            x: point[0],
                            y: point[1],
                        });
                    }
                    interior.push(points);
                }
            }
        }

        let geopoly =
            geometry_rs::Polygon::new_with_rtree_index_opt(exterior, interior, rtree_index);
        return geopoly;
    }

    fn load_georust_poly(data: Vec<u8>) -> geo::Polygon<f64> {
        let file_data: Geometry = serde_json::from_slice(&data).unwrap();
        let mut exterior: Vec<geo::Coord> = vec![];
        let interior: Vec<geo::LineString> = vec![];
        for poly in file_data.coordinates.iter() {
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

    #[test]
    fn test_in_out_polygon() {
        let poly = load_poly(load_az_file(), false);
        let p_in = geometry_rs::Point { x: -112.0, y: 33.0 };
        assert_eq!(poly.contains_point(p_in), true);

        let p_out = geometry_rs::Point {
            x: -114.477539062,
            y: 33.99802726,
        };
        assert_eq!(poly.contains_point(p_out), false);
    }

    #[bench]
    fn poly_contain_point_for_az(b: &mut Bencher) {
        let poly = load_poly(load_az_file(), false);
        let p_in = geometry_rs::Point { x: -112.0, y: 33.0 };
        b.iter(|| {
            let _ = poly.contains_point(p_in);
        });
    }

    #[bench]
    fn georust_poly_contain_point_for_az(b: &mut Bencher) {
        let poly: geo::Polygon = load_georust_poly(load_az_file());
        let p_in: geo::Point = geo::Point::new(-112.0, 33.0);

        b.iter(|| {
            let _ = poly.contains(&p_in);
        });
    }

    #[bench]
    fn poly_contain_point_for_tx(b: &mut Bencher) {
        let poly = load_poly(load_tx_file(), false);
        let p_in = geometry_rs::Point { x: -99.5864, y: 29.0696 };
        b.iter(|| {
            let _ = poly.contains_point(p_in);
        });
    }

    #[bench]
    fn georust_poly_contain_point_for_tx(b: &mut Bencher) {
        let poly: geo::Polygon = load_georust_poly(load_tx_file());
        let p_in: geo::Point = geo::Point::new(-99.5864,29.0696);

        b.iter(|| {
            let _ = poly.contains(&p_in);
        });
    }

}
