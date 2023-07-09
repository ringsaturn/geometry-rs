#![feature(test)]
#[cfg(test)]
mod benches_az_polygon {
    use geometry_rs;
    extern crate test;
    use test::Bencher;

    /* Test data copy from https://github.com/unitedstates/districts/blob/gh-pages/states/AZ/shape.geojson

    Python code to generate the data:
        ```python
        import json

        TPL = """geometry_rs::Point {x: {lng}, y: {lat}},"""

        with open("./az.geojson") as f:
            data = json.loads(f.read())

        coordinates = data["coordinates"][0][0]
        # print(len(coordinates))
        gens = []
        for coord in coordinates:
            # gens.append(TPL.format(lng=coord[0], lat=coord[1]))
            gens.append(TPL.replace("{lng}", str(coord[0])).replace("{lat}", str(coord[1])))
        gen_text = "\n".join(gens)
        print(gen_text)
        ```
    */

    fn load_poly() -> geometry_rs::Polygon {
        let poly = geometry_rs::Polygon::new_with_rtree_index_opt(
            vec![
                geometry_rs::Point {
                    x: -114.635458,
                    y: 34.876902,
                },
                geometry_rs::Point {
                    x: -114.6367680000001,
                    y: 34.885705,
                },
                geometry_rs::Point {
                    x: -114.636725,
                    y: 34.889107,
                },
                geometry_rs::Point {
                    x: -114.635425,
                    y: 34.895192,
                },
                geometry_rs::Point {
                    x: -114.63185,
                    y: 34.903942,
                },
                geometry_rs::Point {
                    x: -114.630877,
                    y: 34.907263,
                },
                geometry_rs::Point {
                    x: -114.630552,
                    y: 34.911852,
                },
                geometry_rs::Point {
                    x: -114.631537,
                    y: 34.916153,
                },
                geometry_rs::Point {
                    x: -114.633237,
                    y: 34.92123,
                },
                geometry_rs::Point {
                    x: -114.633253,
                    y: 34.924608,
                },
                geometry_rs::Point {
                    x: -114.632196,
                    y: 34.930628,
                },
                geometry_rs::Point {
                    x: -114.629753,
                    y: 34.938684,
                },
                geometry_rs::Point {
                    x: -114.629811,
                    y: 34.94481,
                },
                geometry_rs::Point {
                    x: -114.631681,
                    y: 34.95131,
                },
                geometry_rs::Point {
                    x: -114.634274,
                    y: 34.956662,
                },
                geometry_rs::Point {
                    x: -114.634953,
                    y: 34.958918,
                },
                geometry_rs::Point {
                    x: -114.6352370000001,
                    y: 34.965149,
                },
                geometry_rs::Point {
                    x: -114.634607,
                    y: 34.96906,
                },
                geometry_rs::Point {
                    x: -114.629907,
                    y: 34.980791,
                },
                geometry_rs::Point {
                    x: -114.629129,
                    y: 34.986132,
                },
                geometry_rs::Point {
                    x: -114.629443,
                    y: 34.991825,
                },
                geometry_rs::Point {
                    x: -114.6302440000001,
                    y: 34.99464,
                },
                geometry_rs::Point {
                    x: -114.631807,
                    y: 34.998632,
                },
                geometry_rs::Point {
                    x: -114.632665,
                    y: 34.999806,
                },
                geometry_rs::Point {
                    x: -114.6355700000001,
                    y: 35.005933,
                },
                geometry_rs::Point {
                    x: -114.637071,
                    y: 35.010371,
                },
                geometry_rs::Point {
                    x: -114.637769,
                    y: 35.014948,
                },
                geometry_rs::Point {
                    x: -114.63819,
                    y: 35.022069,
                },
                geometry_rs::Point {
                    x: -114.637524,
                    y: 35.027053,
                },
                geometry_rs::Point {
                    x: -114.633715,
                    y: 35.035602,
                },
                geometry_rs::Point {
                    x: -114.629027,
                    y: 35.042531,
                },
                geometry_rs::Point {
                    x: -114.625799,
                    y: 35.045834,
                },
                geometry_rs::Point {
                    x: -114.615902,
                    y: 35.05272,
                },
                geometry_rs::Point {
                    x: -114.6107010000001,
                    y: 35.055458,
                },
                geometry_rs::Point {
                    x: -114.606694,
                    y: 35.058941,
                },
                geometry_rs::Point {
                    x: -114.604715,
                    y: 35.061744,
                },
                geometry_rs::Point {
                    x: -114.603619,
                    y: 35.064226,
                },
                geometry_rs::Point {
                    x: -114.602908,
                    y: 35.068588,
                },
                geometry_rs::Point {
                    x: -114.603175,
                    y: 35.070445,
                },
                geometry_rs::Point {
                    x: -114.604736,
                    y: 35.07483,
                },
                geometry_rs::Point {
                    x: -114.607701,
                    y: 35.078533,
                },
                geometry_rs::Point {
                    x: -114.613132,
                    y: 35.083097,
                },
                geometry_rs::Point {
                    x: -114.61842,
                    y: 35.086539,
                },
                geometry_rs::Point {
                    x: -114.622517,
                    y: 35.088703,
                },
                geometry_rs::Point {
                    x: -114.632053,
                    y: 35.092559,
                },
                geometry_rs::Point {
                    x: -114.63937,
                    y: 35.094733,
                },
                geometry_rs::Point {
                    x: -114.642831,
                    y: 35.096503,
                },
                geometry_rs::Point {
                    x: -114.646579,
                    y: 35.10082,
                },
                geometry_rs::Point {
                    x: -114.646764,
                    y: 35.101868,
                },
                geometry_rs::Point {
                    x: -114.645152,
                    y: 35.104995,
                },
                geometry_rs::Point {
                    x: -114.644354,
                    y: 35.105903,
                },
                geometry_rs::Point {
                    x: -114.641116,
                    y: 35.108401,
                },
                geometry_rs::Point {
                    x: -114.6374320000001,
                    y: 35.112489,
                },
                geometry_rs::Point {
                    x: -114.632282,
                    y: 35.117088,
                },
                geometry_rs::Point {
                    x: -114.628427,
                    y: 35.118943,
                },
                geometry_rs::Point {
                    x: -114.623761,
                    y: 35.120602,
                },
                geometry_rs::Point {
                    x: -114.628993,
                    y: 35.119411,
                },
                geometry_rs::Point {
                    x: -114.6249540000001,
                    y: 35.120742,
                },
                geometry_rs::Point {
                    x: -114.618697,
                    y: 35.121749,
                },
                geometry_rs::Point {
                    x: -114.604007,
                    y: 35.121252,
                },
                geometry_rs::Point {
                    x: -114.60274,
                    y: 35.121666,
                },
                geometry_rs::Point {
                    x: -114.597794,
                    y: 35.121735,
                },
                geometry_rs::Point {
                    x: -114.589787,
                    y: 35.123522,
                },
                geometry_rs::Point {
                    x: -114.584877,
                    y: 35.125194,
                },
                geometry_rs::Point {
                    x: -114.579882,
                    y: 35.127506,
                },
                geometry_rs::Point {
                    x: -114.578263,
                    y: 35.12881,
                },
                geometry_rs::Point {
                    x: -114.577146,
                    y: 35.130982,
                },
                geometry_rs::Point {
                    x: -114.574411,
                    y: 35.13495,
                },
                geometry_rs::Point {
                    x: -114.572597,
                    y: 35.139557,
                },
                geometry_rs::Point {
                    x: -114.573706,
                    y: 35.142698,
                },
                geometry_rs::Point {
                    x: -114.573879,
                    y: 35.145351,
                },
                geometry_rs::Point {
                    x: -114.569529,
                    y: 35.162317,
                },
                geometry_rs::Point {
                    x: -114.56876,
                    y: 35.172195,
                },
                geometry_rs::Point {
                    x: -114.569214,
                    y: 35.17289,
                },
                geometry_rs::Point {
                    x: -114.568989,
                    y: 35.175085,
                },
                geometry_rs::Point {
                    x: -114.569258,
                    y: 35.183424,
                },
                geometry_rs::Point {
                    x: -114.569653,
                    y: 35.186267,
                },
                geometry_rs::Point {
                    x: -114.571404,
                    y: 35.191026,
                },
                geometry_rs::Point {
                    x: -114.572084,
                    y: 35.200794,
                },
                geometry_rs::Point {
                    x: -114.574037,
                    y: 35.20379,
                },
                geometry_rs::Point {
                    x: -114.574233,
                    y: 35.205481,
                },
                geometry_rs::Point {
                    x: -114.574958,
                    y: 35.206714,
                },
                geometry_rs::Point {
                    x: -114.578581,
                    y: 35.208113,
                },
                geometry_rs::Point {
                    x: -114.579535,
                    y: 35.208911,
                },
                geometry_rs::Point {
                    x: -114.5798970000001,
                    y: 35.21097,
                },
                geometry_rs::Point {
                    x: -114.580312,
                    y: 35.220095,
                },
                geometry_rs::Point {
                    x: -114.583523,
                    y: 35.230348,
                },
                geometry_rs::Point {
                    x: -114.58248,
                    y: 35.233173,
                },
                geometry_rs::Point {
                    x: -114.582842,
                    y: 35.238703,
                },
                geometry_rs::Point {
                    x: -114.584993,
                    y: 35.242717,
                },
                geometry_rs::Point {
                    x: -114.586053,
                    y: 35.248891,
                },
                geometry_rs::Point {
                    x: -114.585714,
                    y: 35.253145,
                },
                geometry_rs::Point {
                    x: -114.585768,
                    y: 35.257743,
                },
                geometry_rs::Point {
                    x: -114.586604,
                    y: 35.262386,
                },
                geometry_rs::Point {
                    x: -114.587497,
                    y: 35.265473,
                },
                geometry_rs::Point {
                    x: -114.590513,
                    y: 35.272334,
                },
                geometry_rs::Point {
                    x: -114.593247,
                    y: 35.284361,
                },
                geometry_rs::Point {
                    x: -114.595705,
                    y: 35.289939,
                },
                geometry_rs::Point {
                    x: -114.596682,
                    y: 35.294557,
                },
                geometry_rs::Point {
                    x: -114.597268,
                    y: 35.299565,
                },
                geometry_rs::Point {
                    x: -114.59721,
                    y: 35.303223,
                },
                geometry_rs::Point {
                    x: -114.595163,
                    y: 35.321883,
                },
                geometry_rs::Point {
                    x: -114.595553,
                    y: 35.326547,
                },
                geometry_rs::Point {
                    x: -114.599771,
                    y: 35.34111,
                },
                geometry_rs::Point {
                    x: -114.604607,
                    y: 35.355239,
                },
                geometry_rs::Point {
                    x: -114.606173,
                    y: 35.359651,
                },
                geometry_rs::Point {
                    x: -114.611206,
                    y: 35.370119,
                },
                geometry_rs::Point {
                    x: -114.617698,
                    y: 35.380131,
                },
                geometry_rs::Point {
                    x: -114.618257,
                    y: 35.382646,
                },
                geometry_rs::Point {
                    x: -114.618984,
                    y: 35.389391,
                },
                geometry_rs::Point {
                    x: -114.620887,
                    y: 35.396867,
                },
                geometry_rs::Point {
                    x: -114.621783,
                    y: 35.39945,
                },
                geometry_rs::Point {
                    x: -114.625702,
                    y: 35.407976,
                },
                geometry_rs::Point {
                    x: -114.626765,
                    y: 35.409644,
                },
                geometry_rs::Point {
                    x: -114.6290610000001,
                    y: 35.411175,
                },
                geometry_rs::Point {
                    x: -114.65208,
                    y: 35.430134,
                },
                geometry_rs::Point {
                    x: -114.653817,
                    y: 35.432853,
                },
                geometry_rs::Point {
                    x: -114.654295,
                    y: 35.436854,
                },
                geometry_rs::Point {
                    x: -114.658105,
                    y: 35.441835,
                },
                geometry_rs::Point {
                    x: -114.661747,
                    y: 35.444735,
                },
                geometry_rs::Point {
                    x: -114.662896,
                    y: 35.446449,
                },
                geometry_rs::Point {
                    x: -114.663934,
                    y: 35.449466,
                },
                geometry_rs::Point {
                    x: -114.664215,
                    y: 35.451707,
                },
                geometry_rs::Point {
                    x: -114.6638800000001,
                    y: 35.454657,
                },
                geometry_rs::Point {
                    x: -114.664217,
                    y: 35.455845,
                },
                geometry_rs::Point {
                    x: -114.665142,
                    y: 35.457331,
                },
                geometry_rs::Point {
                    x: -114.666151,
                    y: 35.458198,
                },
                geometry_rs::Point {
                    x: -114.667217,
                    y: 35.46037,
                },
                geometry_rs::Point {
                    x: -114.6667690000001,
                    y: 35.462085,
                },
                geometry_rs::Point {
                    x: -114.6657900000001,
                    y: 35.463915,
                },
                geometry_rs::Point {
                    x: -114.665651,
                    y: 35.466911,
                },
                geometry_rs::Point {
                    x: -114.665988,
                    y: 35.467985,
                },
                geometry_rs::Point {
                    x: -114.667389,
                    y: 35.469904,
                },
                geometry_rs::Point {
                    x: -114.67235,
                    y: 35.47374,
                },
                geometry_rs::Point {
                    x: -114.673164,
                    y: 35.474814,
                },
                geometry_rs::Point {
                    x: -114.673585,
                    y: 35.475843,
                },
                geometry_rs::Point {
                    x: -114.6734730000001,
                    y: 35.476849,
                },
                geometry_rs::Point {
                    x: -114.672074,
                    y: 35.479709,
                },
                geometry_rs::Point {
                    x: -114.671794,
                    y: 35.480806,
                },
                geometry_rs::Point {
                    x: -114.671907,
                    y: 35.482087,
                },
                geometry_rs::Point {
                    x: -114.673534,
                    y: 35.485675,
                },
                geometry_rs::Point {
                    x: -114.6768150000001,
                    y: 35.489787,
                },
                geometry_rs::Point {
                    x: -114.6767040000001,
                    y: 35.491845,
                },
                geometry_rs::Point {
                    x: -114.676257,
                    y: 35.493103,
                },
                geometry_rs::Point {
                    x: -114.677743,
                    y: 35.495182,
                },
                geometry_rs::Point {
                    x: -114.678642,
                    y: 35.497628,
                },
                geometry_rs::Point {
                    x: -114.678587,
                    y: 35.499846,
                },
                geometry_rs::Point {
                    x: -114.6788920000001,
                    y: 35.501276,
                },
                geometry_rs::Point {
                    x: -114.67748,
                    y: 35.510948,
                },
                geometry_rs::Point {
                    x: -114.677143,
                    y: 35.512945,
                },
                geometry_rs::Point {
                    x: -114.675685,
                    y: 35.51563,
                },
                geometry_rs::Point {
                    x: -114.672767,
                    y: 35.518428,
                },
                geometry_rs::Point {
                    x: -114.66954,
                    y: 35.52079,
                },
                geometry_rs::Point {
                    x: -114.668586,
                    y: 35.521225,
                },
                geometry_rs::Point {
                    x: -114.666565,
                    y: 35.520993,
                },
                geometry_rs::Point {
                    x: -114.6646010000001,
                    y: 35.521519,
                },
                geometry_rs::Point {
                    x: -114.6639830000001,
                    y: 35.522161,
                },
                geometry_rs::Point {
                    x: -114.661682,
                    y: 35.526682,
                },
                geometry_rs::Point {
                    x: -114.659886,
                    y: 35.527919,
                },
                geometry_rs::Point {
                    x: -114.657753,
                    y: 35.530741,
                },
                geometry_rs::Point {
                    x: -114.657163,
                    y: 35.532301,
                },
                geometry_rs::Point {
                    x: -114.65677,
                    y: 35.534964,
                },
                geometry_rs::Point {
                    x: -114.657809,
                    y: 35.536963,
                },
                geometry_rs::Point {
                    x: -114.660335,
                    y: 35.540433,
                },
                geometry_rs::Point {
                    x: -114.661457,
                    y: 35.544062,
                },
                geometry_rs::Point {
                    x: -114.66157,
                    y: 35.545692,
                },
                geometry_rs::Point {
                    x: -114.66112,
                    y: 35.549021,
                },
                geometry_rs::Point {
                    x: -114.661963,
                    y: 35.552604,
                },
                geometry_rs::Point {
                    x: -114.661963,
                    y: 35.555887,
                },
                geometry_rs::Point {
                    x: -114.663451,
                    y: 35.559884,
                },
                geometry_rs::Point {
                    x: -114.663535,
                    y: 35.560963,
                },
                geometry_rs::Point {
                    x: -114.662805,
                    y: 35.564268,
                },
                geometry_rs::Point {
                    x: -114.6639,
                    y: 35.56629,
                },
                geometry_rs::Point {
                    x: -114.6644330000001,
                    y: 35.568426,
                },
                geometry_rs::Point {
                    x: -114.666231,
                    y: 35.571642,
                },
                geometry_rs::Point {
                    x: -114.668393,
                    y: 35.574331,
                },
                geometry_rs::Point {
                    x: -114.670022,
                    y: 35.575596,
                },
                geometry_rs::Point {
                    x: -114.671567,
                    y: 35.576217,
                },
                geometry_rs::Point {
                    x: -114.674881,
                    y: 35.578379,
                },
                geometry_rs::Point {
                    x: -114.675751,
                    y: 35.579459,
                },
                geometry_rs::Point {
                    x: -114.675667,
                    y: 35.580033,
                },
                geometry_rs::Point {
                    x: -114.670191,
                    y: 35.583471,
                },
                geometry_rs::Point {
                    x: -114.664209,
                    y: 35.585944,
                },
                geometry_rs::Point {
                    x: -114.660558,
                    y: 35.586583,
                },
                geometry_rs::Point {
                    x: -114.659238,
                    y: 35.587477,
                },
                geometry_rs::Point {
                    x: -114.654518,
                    y: 35.596609,
                },
                geometry_rs::Point {
                    x: -114.6539000000001,
                    y: 35.598491,
                },
                geometry_rs::Point {
                    x: -114.653731,
                    y: 35.600373,
                },
                geometry_rs::Point {
                    x: -114.654489,
                    y: 35.605173,
                },
                geometry_rs::Point {
                    x: -114.653618,
                    y: 35.607192,
                },
                geometry_rs::Point {
                    x: -114.653534,
                    y: 35.609672,
                },
                geometry_rs::Point {
                    x: -114.653927,
                    y: 35.611739,
                },
                geometry_rs::Point {
                    x: -114.655219,
                    y: 35.614059,
                },
                geometry_rs::Point {
                    x: -114.657241,
                    y: 35.617046,
                },
                geometry_rs::Point {
                    x: -114.659461,
                    y: 35.619552,
                },
                geometry_rs::Point {
                    x: -114.660641,
                    y: 35.620334,
                },
                geometry_rs::Point {
                    x: -114.663647,
                    y: 35.620773,
                },
                geometry_rs::Point {
                    x: -114.665389,
                    y: 35.621556,
                },
                geometry_rs::Point {
                    x: -114.666682,
                    y: 35.623073,
                },
                geometry_rs::Point {
                    x: -114.668087,
                    y: 35.627115,
                },
                geometry_rs::Point {
                    x: -114.6690150000001,
                    y: 35.628861,
                },
                geometry_rs::Point {
                    x: -114.672134,
                    y: 35.633365,
                },
                geometry_rs::Point {
                    x: -114.675001,
                    y: 35.638304,
                },
                geometry_rs::Point {
                    x: -114.677615,
                    y: 35.641774,
                },
                geometry_rs::Point {
                    x: -114.6794150000001,
                    y: 35.643429,
                },
                geometry_rs::Point {
                    x: -114.686133,
                    y: 35.647522,
                },
                geometry_rs::Point {
                    x: -114.689001,
                    y: 35.65028,
                },
                geometry_rs::Point {
                    x: -114.689507,
                    y: 35.651429,
                },
                geometry_rs::Point {
                    x: -114.689226,
                    y: 35.652898,
                },
                geometry_rs::Point {
                    x: -114.6904940000001,
                    y: 35.662657,
                },
                geometry_rs::Point {
                    x: -114.690214,
                    y: 35.665159,
                },
                geometry_rs::Point {
                    x: -114.686055,
                    y: 35.670642,
                },
                geometry_rs::Point {
                    x: -114.682317,
                    y: 35.677825,
                },
                geometry_rs::Point {
                    x: -114.680827,
                    y: 35.682255,
                },
                geometry_rs::Point {
                    x: -114.6806310000001,
                    y: 35.684046,
                },
                geometry_rs::Point {
                    x: -114.6809970000001,
                    y: 35.685929,
                },
                geometry_rs::Point {
                    x: -114.6826570000001,
                    y: 35.688571,
                },
                geometry_rs::Point {
                    x: -114.691263,
                    y: 35.693125,
                },
                geometry_rs::Point {
                    x: -114.696214,
                    y: 35.69655,
                },
                geometry_rs::Point {
                    x: -114.701416,
                    y: 35.701084,
                },
                geometry_rs::Point {
                    x: -114.703608,
                    y: 35.703922,
                },
                geometry_rs::Point {
                    x: -114.704501,
                    y: 35.705993,
                },
                geometry_rs::Point {
                    x: -114.704959,
                    y: 35.706366,
                },
                geometry_rs::Point {
                    x: -114.704842,
                    y: 35.706744,
                },
                geometry_rs::Point {
                    x: -114.705597,
                    y: 35.708274,
                },
                geometry_rs::Point {
                    x: -114.7053470000001,
                    y: 35.708344,
                },
                geometry_rs::Point {
                    x: -114.705447,
                    y: 35.711757,
                },
                geometry_rs::Point {
                    x: -114.699405,
                    y: 35.726929,
                },
                geometry_rs::Point {
                    x: -114.697859,
                    y: 35.731657,
                },
                geometry_rs::Point {
                    x: -114.69654,
                    y: 35.738934,
                },
                geometry_rs::Point {
                    x: -114.6964,
                    y: 35.742653,
                },
                geometry_rs::Point {
                    x: -114.696655,
                    y: 35.746143,
                },
                geometry_rs::Point {
                    x: -114.697585,
                    y: 35.748417,
                },
                geometry_rs::Point {
                    x: -114.697726,
                    y: 35.750966,
                },
                geometry_rs::Point {
                    x: -114.696854,
                    y: 35.752756,
                },
                geometry_rs::Point {
                    x: -114.696546,
                    y: 35.754638,
                },
                geometry_rs::Point {
                    x: -114.694267,
                    y: 35.756633,
                },
                geometry_rs::Point {
                    x: -114.694717,
                    y: 35.757897,
                },
                geometry_rs::Point {
                    x: -114.69742,
                    y: 35.760677,
                },
                geometry_rs::Point {
                    x: -114.700266,
                    y: 35.766879,
                },
                geometry_rs::Point {
                    x: -114.701027,
                    y: 35.76968,
                },
                geometry_rs::Point {
                    x: -114.70117,
                    y: 35.774112,
                },
                geometry_rs::Point {
                    x: -114.6990360000001,
                    y: 35.788046,
                },
                geometry_rs::Point {
                    x: -114.6993180000001,
                    y: 35.79048,
                },
                geometry_rs::Point {
                    x: -114.703178,
                    y: 35.794685,
                },
                geometry_rs::Point {
                    x: -114.705827,
                    y: 35.798889,
                },
                geometry_rs::Point {
                    x: -114.71149,
                    y: 35.80438,
                },
                geometry_rs::Point {
                    x: -114.712026,
                    y: 35.805529,
                },
                geometry_rs::Point {
                    x: -114.710534,
                    y: 35.807525,
                },
                geometry_rs::Point {
                    x: -114.709324,
                    y: 35.81005,
                },
                geometry_rs::Point {
                    x: -114.70634,
                    y: 35.812022,
                },
                geometry_rs::Point {
                    x: -114.703665,
                    y: 35.814614,
                },
                geometry_rs::Point {
                    x: -114.700654,
                    y: 35.822004,
                },
                geometry_rs::Point {
                    x: -114.697276,
                    y: 35.826776,
                },
                geometry_rs::Point {
                    x: -114.69553,
                    y: 35.829897,
                },
                geometry_rs::Point {
                    x: -114.6952770000001,
                    y: 35.831091,
                },
                geometry_rs::Point {
                    x: -114.695249,
                    y: 35.832285,
                },
                geometry_rs::Point {
                    x: -114.695757,
                    y: 35.833387,
                },
                geometry_rs::Point {
                    x: -114.701478,
                    y: 35.839316,
                },
                geometry_rs::Point {
                    x: -114.702293,
                    y: 35.840792,
                },
                geometry_rs::Point {
                    x: -114.702339,
                    y: 35.842151,
                },
                geometry_rs::Point {
                    x: -114.703527,
                    y: 35.841845,
                },
                geometry_rs::Point {
                    x: -114.704173,
                    y: 35.842669,
                },
                geometry_rs::Point {
                    x: -114.7042030000001,
                    y: 35.844274,
                },
                geometry_rs::Point {
                    x: -114.706288,
                    y: 35.846218,
                },
                geometry_rs::Point {
                    x: -114.706532,
                    y: 35.849027,
                },
                geometry_rs::Point {
                    x: -114.705856,
                    y: 35.850508,
                },
                geometry_rs::Point {
                    x: -114.703599,
                    y: 35.852595,
                },
                geometry_rs::Point {
                    x: -114.701904,
                    y: 35.853223,
                },
                geometry_rs::Point {
                    x: -114.696581,
                    y: 35.853727,
                },
                geometry_rs::Point {
                    x: -114.69437,
                    y: 35.854463,
                },
                geometry_rs::Point {
                    x: -114.693446,
                    y: 35.855125,
                },
                geometry_rs::Point {
                    x: -114.691456,
                    y: 35.858661,
                },
                geometry_rs::Point {
                    x: -114.6877980000001,
                    y: 35.860728,
                },
                geometry_rs::Point {
                    x: -114.68205,
                    y: 35.86295,
                },
                geometry_rs::Point {
                    x: -114.678186,
                    y: 35.863311,
                },
                geometry_rs::Point {
                    x: -114.672289,
                    y: 35.865011,
                },
                geometry_rs::Point {
                    x: -114.66968,
                    y: 35.865036,
                },
                geometry_rs::Point {
                    x: -114.6674710000001,
                    y: 35.867061,
                },
                geometry_rs::Point {
                    x: -114.662623,
                    y: 35.869213,
                },
                geometry_rs::Point {
                    x: -114.661636,
                    y: 35.870545,
                },
                geometry_rs::Point {
                    x: -114.661636,
                    y: 35.871233,
                },
                geometry_rs::Point {
                    x: -114.663214,
                    y: 35.873692,
                },
                geometry_rs::Point {
                    x: -114.668145,
                    y: 35.875201,
                },
                geometry_rs::Point {
                    x: -114.672009,
                    y: 35.878018,
                },
                geometry_rs::Point {
                    x: -114.678972,
                    y: 35.88551,
                },
                geometry_rs::Point {
                    x: -114.693602,
                    y: 35.895311,
                },
                geometry_rs::Point {
                    x: -114.69454,
                    y: 35.896587,
                },
                geometry_rs::Point {
                    x: -114.696064,
                    y: 35.896464,
                },
                geometry_rs::Point {
                    x: -114.694928,
                    y: 35.897594,
                },
                geometry_rs::Point {
                    x: -114.6961320000001,
                    y: 35.898662,
                },
                geometry_rs::Point {
                    x: -114.697558,
                    y: 35.89936,
                },
                geometry_rs::Point {
                    x: -114.7002580000001,
                    y: 35.901757,
                },
                geometry_rs::Point {
                    x: -114.700769,
                    y: 35.903064,
                },
                geometry_rs::Point {
                    x: -114.703538,
                    y: 35.906707,
                },
                geometry_rs::Point {
                    x: -114.705119,
                    y: 35.907637,
                },
                geometry_rs::Point {
                    x: -114.705991,
                    y: 35.908598,
                },
                geometry_rs::Point {
                    x: -114.7057140000001,
                    y: 35.909316,
                },
                geometry_rs::Point {
                    x: -114.706767,
                    y: 35.90895,
                },
                geometry_rs::Point {
                    x: -114.708112,
                    y: 35.909933,
                },
                geometry_rs::Point {
                    x: -114.709187,
                    y: 35.916827,
                },
                geometry_rs::Point {
                    x: -114.707784,
                    y: 35.916993,
                },
                geometry_rs::Point {
                    x: -114.707398,
                    y: 35.918057,
                },
                geometry_rs::Point {
                    x: -114.70788,
                    y: 35.919207,
                },
                geometry_rs::Point {
                    x: -114.7073290000001,
                    y: 35.926177,
                },
                geometry_rs::Point {
                    x: -114.707603,
                    y: 35.92795,
                },
                geometry_rs::Point {
                    x: -114.712965,
                    y: 35.932159,
                },
                geometry_rs::Point {
                    x: -114.712756,
                    y: 35.932639,
                },
                geometry_rs::Point {
                    x: -114.713413,
                    y: 35.9319,
                },
                geometry_rs::Point {
                    x: -114.713312,
                    y: 35.933844,
                },
                geometry_rs::Point {
                    x: -114.729762,
                    y: 35.959895,
                },
                geometry_rs::Point {
                    x: -114.7284960000001,
                    y: 35.960395,
                },
                geometry_rs::Point {
                    x: -114.728666,
                    y: 35.961757,
                },
                geometry_rs::Point {
                    x: -114.7300900000001,
                    y: 35.962691,
                },
                geometry_rs::Point {
                    x: -114.732456,
                    y: 35.965891,
                },
                geometry_rs::Point {
                    x: -114.736195,
                    y: 35.969421,
                },
                geometry_rs::Point {
                    x: -114.740536,
                    y: 35.975545,
                },
                geometry_rs::Point {
                    x: -114.743494,
                    y: 35.983553,
                },
                geometry_rs::Point {
                    x: -114.743638,
                    y: 35.985785,
                },
                geometry_rs::Point {
                    x: -114.743117,
                    y: 35.987387,
                },
                geometry_rs::Point {
                    x: -114.740043,
                    y: 35.990534,
                },
                geometry_rs::Point {
                    x: -114.739318,
                    y: 35.991804,
                },
                geometry_rs::Point {
                    x: -114.740544,
                    y: 35.994853,
                },
                geometry_rs::Point {
                    x: -114.740815,
                    y: 35.997464,
                },
                geometry_rs::Point {
                    x: -114.741536,
                    y: 35.99969,
                },
                geometry_rs::Point {
                    x: -114.741679,
                    y: 36.002283,
                },
                geometry_rs::Point {
                    x: -114.743163,
                    y: 36.006722,
                },
                geometry_rs::Point {
                    x: -114.743005,
                    y: 36.00845,
                },
                geometry_rs::Point {
                    x: -114.740866,
                    y: 36.012928,
                },
                geometry_rs::Point {
                    x: -114.738555,
                    y: 36.015223,
                },
                geometry_rs::Point {
                    x: -114.7288740000001,
                    y: 36.021387,
                },
                geometry_rs::Point {
                    x: -114.723324,
                    y: 36.026588,
                },
                geometry_rs::Point {
                    x: -114.722214,
                    y: 36.027964,
                },
                geometry_rs::Point {
                    x: -114.722096,
                    y: 36.028952,
                },
                geometry_rs::Point {
                    x: -114.722742,
                    y: 36.030286,
                },
                geometry_rs::Point {
                    x: -114.723673,
                    y: 36.03123,
                },
                geometry_rs::Point {
                    x: -114.727602,
                    y: 36.033099,
                },
                geometry_rs::Point {
                    x: -114.730563,
                    y: 36.036207,
                },
                geometry_rs::Point {
                    x: -114.7334170000001,
                    y: 36.037913,
                },
                geometry_rs::Point {
                    x: -114.735739,
                    y: 36.038033,
                },
                geometry_rs::Point {
                    x: -114.740018,
                    y: 36.037467,
                },
                geometry_rs::Point {
                    x: -114.7412620000001,
                    y: 36.038044,
                },
                geometry_rs::Point {
                    x: -114.742105,
                    y: 36.039792,
                },
                geometry_rs::Point {
                    x: -114.742661,
                    y: 36.042573,
                },
                geometry_rs::Point {
                    x: -114.742479,
                    y: 36.045697,
                },
                geometry_rs::Point {
                    x: -114.741677,
                    y: 36.047877,
                },
                geometry_rs::Point {
                    x: -114.735701,
                    y: 36.053393,
                },
                geometry_rs::Point {
                    x: -114.73508,
                    y: 36.054435,
                },
                geometry_rs::Point {
                    x: -114.735285,
                    y: 36.056648,
                },
                geometry_rs::Point {
                    x: -114.7360230000001,
                    y: 36.059063,
                },
                geometry_rs::Point {
                    x: -114.74006,
                    y: 36.062437,
                },
                geometry_rs::Point {
                    x: -114.7422,
                    y: 36.067833,
                },
                geometry_rs::Point {
                    x: -114.742138,
                    y: 36.068676,
                },
                geometry_rs::Point {
                    x: -114.743542,
                    y: 36.071037,
                },
                geometry_rs::Point {
                    x: -114.748891,
                    y: 36.074981,
                },
                geometry_rs::Point {
                    x: -114.75057,
                    y: 36.08033,
                },
                geometry_rs::Point {
                    x: -114.754032,
                    y: 36.083093,
                },
                geometry_rs::Point {
                    x: -114.754681,
                    y: 36.085052,
                },
                geometry_rs::Point {
                    x: -114.754508,
                    y: 36.086171,
                },
                geometry_rs::Point {
                    x: -114.752836,
                    y: 36.089393,
                },
                geometry_rs::Point {
                    x: -114.750095,
                    y: 36.092275,
                },
                geometry_rs::Point {
                    x: -114.7489130000001,
                    y: 36.095183,
                },
                geometry_rs::Point {
                    x: -114.7374970000001,
                    y: 36.103102,
                },
                geometry_rs::Point {
                    x: -114.734857,
                    y: 36.104426,
                },
                geometry_rs::Point {
                    x: -114.718257,
                    y: 36.107164,
                },
                geometry_rs::Point {
                    x: -114.709269,
                    y: 36.107396,
                },
                geometry_rs::Point {
                    x: -114.706091,
                    y: 36.108239,
                },
                geometry_rs::Point {
                    x: -114.703737,
                    y: 36.108348,
                },
                geometry_rs::Point {
                    x: -114.696981,
                    y: 36.110297,
                },
                geometry_rs::Point {
                    x: -114.6936550000001,
                    y: 36.112482,
                },
                geometry_rs::Point {
                    x: -114.691631,
                    y: 36.112535,
                },
                geometry_rs::Point {
                    x: -114.6880740000001,
                    y: 36.111457,
                },
                geometry_rs::Point {
                    x: -114.6844260000001,
                    y: 36.109472,
                },
                geometry_rs::Point {
                    x: -114.6818470000001,
                    y: 36.109192,
                },
                geometry_rs::Point {
                    x: -114.6797750000001,
                    y: 36.109874,
                },
                geometry_rs::Point {
                    x: -114.678375,
                    y: 36.110815,
                },
                geometry_rs::Point {
                    x: -114.675106,
                    y: 36.114111,
                },
                geometry_rs::Point {
                    x: -114.671867,
                    y: 36.115964,
                },
                geometry_rs::Point {
                    x: -114.6643430000001,
                    y: 36.1163,
                },
                geometry_rs::Point {
                    x: -114.662144,
                    y: 36.117742,
                },
                geometry_rs::Point {
                    x: -114.660448,
                    y: 36.119999,
                },
                geometry_rs::Point {
                    x: -114.658131,
                    y: 36.124127,
                },
                geometry_rs::Point {
                    x: -114.655512,
                    y: 36.126187,
                },
                geometry_rs::Point {
                    x: -114.645728,
                    y: 36.131995,
                },
                geometry_rs::Point {
                    x: -114.641976,
                    y: 36.13373,
                },
                geometry_rs::Point {
                    x: -114.640125,
                    y: 36.135126,
                },
                geometry_rs::Point {
                    x: -114.636862,
                    y: 36.135552,
                },
                geometry_rs::Point {
                    x: -114.635809,
                    y: 36.13617,
                },
                geometry_rs::Point {
                    x: -114.630474,
                    y: 36.142218,
                },
                geometry_rs::Point {
                    x: -114.628462,
                    y: 36.141822,
                },
                geometry_rs::Point {
                    x: -114.627079,
                    y: 36.140761,
                },
                geometry_rs::Point {
                    x: -114.623837,
                    y: 36.137144,
                },
                geometry_rs::Point {
                    x: -114.620605,
                    y: 36.131759,
                },
                geometry_rs::Point {
                    x: -114.618429,
                    y: 36.130328,
                },
                geometry_rs::Point {
                    x: -114.615455,
                    y: 36.129653,
                },
                geometry_rs::Point {
                    x: -114.61324,
                    y: 36.130266,
                },
                geometry_rs::Point {
                    x: -114.609288,
                    y: 36.132229,
                },
                geometry_rs::Point {
                    x: -114.596474,
                    y: 36.141537,
                },
                geometry_rs::Point {
                    x: -114.5930350000001,
                    y: 36.142674,
                },
                geometry_rs::Point {
                    x: -114.589828,
                    y: 36.143192,
                },
                geometry_rs::Point {
                    x: -114.583716,
                    y: 36.14556,
                },
                geometry_rs::Point {
                    x: -114.580707,
                    y: 36.145987,
                },
                geometry_rs::Point {
                    x: -114.578828,
                    y: 36.147175,
                },
                geometry_rs::Point {
                    x: -114.57706,
                    y: 36.148845,
                },
                geometry_rs::Point {
                    x: -114.57109,
                    y: 36.151099,
                },
                geometry_rs::Point {
                    x: -114.561173,
                    y: 36.150921,
                },
                geometry_rs::Point {
                    x: -114.556162,
                    y: 36.15247,
                },
                geometry_rs::Point {
                    x: -114.5487420000001,
                    y: 36.150697,
                },
                geometry_rs::Point {
                    x: -114.543232,
                    y: 36.151871,
                },
                geometry_rs::Point {
                    x: -114.539233,
                    y: 36.151764,
                },
                geometry_rs::Point {
                    x: -114.534478,
                    y: 36.15023,
                },
                geometry_rs::Point {
                    x: -114.532924,
                    y: 36.149282,
                },
                geometry_rs::Point {
                    x: -114.532308,
                    y: 36.14804,
                },
                geometry_rs::Point {
                    x: -114.531091,
                    y: 36.147644,
                },
                geometry_rs::Point {
                    x: -114.52621,
                    y: 36.148177,
                },
                geometry_rs::Point {
                    x: -114.51428,
                    y: 36.150795,
                },
                geometry_rs::Point {
                    x: -114.511218,
                    y: 36.150576,
                },
                geometry_rs::Point {
                    x: -114.5081040000001,
                    y: 36.149713,
                },
                geometry_rs::Point {
                    x: -114.501049,
                    y: 36.144516,
                },
                geometry_rs::Point {
                    x: -114.500236,
                    y: 36.143226,
                },
                geometry_rs::Point {
                    x: -114.499992,
                    y: 36.141594,
                },
                geometry_rs::Point {
                    x: -114.500339,
                    y: 36.1407,
                },
                geometry_rs::Point {
                    x: -114.50108,
                    y: 36.14006,
                },
                geometry_rs::Point {
                    x: -114.50515,
                    y: 36.138078,
                },
                geometry_rs::Point {
                    x: -114.507175,
                    y: 36.13634,
                },
                geometry_rs::Point {
                    x: -114.50921,
                    y: 36.133247,
                },
                geometry_rs::Point {
                    x: -114.508467,
                    y: 36.129913,
                },
                geometry_rs::Point {
                    x: -114.507201,
                    y: 36.128484,
                },
                geometry_rs::Point {
                    x: -114.504715,
                    y: 36.127188,
                },
                geometry_rs::Point {
                    x: -114.501798,
                    y: 36.126556,
                },
                geometry_rs::Point {
                    x: -114.498849,
                    y: 36.126612,
                },
                geometry_rs::Point {
                    x: -114.487635,
                    y: 36.128656,
                },
                geometry_rs::Point {
                    x: -114.483827,
                    y: 36.12972,
                },
                geometry_rs::Point {
                    x: -114.478248,
                    y: 36.132683,
                },
                geometry_rs::Point {
                    x: -114.468674,
                    y: 36.138889,
                },
                geometry_rs::Point {
                    x: -114.465579,
                    y: 36.139496,
                },
                geometry_rs::Point {
                    x: -114.4626,
                    y: 36.139644,
                },
                geometry_rs::Point {
                    x: -114.458945,
                    y: 36.139214,
                },
                geometry_rs::Point {
                    x: -114.456487,
                    y: 36.138032,
                },
                geometry_rs::Point {
                    x: -114.45511,
                    y: 36.136372,
                },
                geometry_rs::Point {
                    x: -114.453798,
                    y: 36.133586,
                },
                geometry_rs::Point {
                    x: -114.451331,
                    y: 36.129831,
                },
                geometry_rs::Point {
                    x: -114.447135,
                    y: 36.126022,
                },
                geometry_rs::Point {
                    x: -114.445042,
                    y: 36.125346,
                },
                geometry_rs::Point {
                    x: -114.443736,
                    y: 36.125593,
                },
                geometry_rs::Point {
                    x: -114.435507,
                    y: 36.130057,
                },
                geometry_rs::Point {
                    x: -114.423114,
                    y: 36.13735,
                },
                geometry_rs::Point {
                    x: -114.418193,
                    y: 36.142771,
                },
                geometry_rs::Point {
                    x: -114.415253,
                    y: 36.145123,
                },
                geometry_rs::Point {
                    x: -114.4124910000001,
                    y: 36.146511,
                },
                geometry_rs::Point {
                    x: -114.40914,
                    y: 36.147,
                },
                geometry_rs::Point {
                    x: -114.405624,
                    y: 36.146983,
                },
                geometry_rs::Point {
                    x: -114.398373,
                    y: 36.145799,
                },
                geometry_rs::Point {
                    x: -114.381479,
                    y: 36.141349,
                },
                geometry_rs::Point {
                    x: -114.379976,
                    y: 36.141388,
                },
                geometry_rs::Point {
                    x: -114.375278,
                    y: 36.143592,
                },
                geometry_rs::Point {
                    x: -114.373745,
                    y: 36.143722,
                },
                geometry_rs::Point {
                    x: -114.370181,
                    y: 36.142624,
                },
                geometry_rs::Point {
                    x: -114.368551,
                    y: 36.140892,
                },
                geometry_rs::Point {
                    x: -114.367381,
                    y: 36.13852,
                },
                geometry_rs::Point {
                    x: -114.365529,
                    y: 36.136306,
                },
                geometry_rs::Point {
                    x: -114.364499,
                    y: 36.134072,
                },
                geometry_rs::Point {
                    x: -114.358968,
                    y: 36.127795,
                },
                geometry_rs::Point {
                    x: -114.348592,
                    y: 36.121147,
                },
                geometry_rs::Point {
                    x: -114.3451,
                    y: 36.118556,
                },
                geometry_rs::Point {
                    x: -114.342601,
                    y: 36.115878,
                },
                geometry_rs::Point {
                    x: -114.34095,
                    y: 36.113457,
                },
                geometry_rs::Point {
                    x: -114.338815,
                    y: 36.111309,
                },
                geometry_rs::Point {
                    x: -114.337264,
                    y: 36.110428,
                },
                geometry_rs::Point {
                    x: -114.334632,
                    y: 36.106784,
                },
                geometry_rs::Point {
                    x: -114.333587,
                    y: 36.106342,
                },
                geometry_rs::Point {
                    x: -114.328801,
                    y: 36.105902,
                },
                geometry_rs::Point {
                    x: -114.325814,
                    y: 36.103933,
                },
                geometry_rs::Point {
                    x: -114.325539,
                    y: 36.102989,
                },
                geometry_rs::Point {
                    x: -114.3234580000001,
                    y: 36.101186,
                },
                geometry_rs::Point {
                    x: -114.320866,
                    y: 36.096463,
                },
                geometry_rs::Point {
                    x: -114.316983,
                    y: 36.093409,
                },
                geometry_rs::Point {
                    x: -114.313086,
                    y: 36.088816,
                },
                geometry_rs::Point {
                    x: -114.306939,
                    y: 36.082487,
                },
                geometry_rs::Point {
                    x: -114.304171,
                    y: 36.07558,
                },
                geometry_rs::Point {
                    x: -114.304384,
                    y: 36.074019,
                },
                geometry_rs::Point {
                    x: -114.305853,
                    y: 36.071478,
                },
                geometry_rs::Point {
                    x: -114.307485,
                    y: 36.069672,
                },
                geometry_rs::Point {
                    x: -114.31242,
                    y: 36.066117,
                },
                geometry_rs::Point {
                    x: -114.3136,
                    y: 36.064148,
                },
                geometry_rs::Point {
                    x: -114.314328,
                    y: 36.062016,
                },
                geometry_rs::Point {
                    x: -114.314427,
                    y: 36.060523,
                },
                geometry_rs::Point {
                    x: -114.313591,
                    y: 36.059048,
                },
                geometry_rs::Point {
                    x: -114.311904,
                    y: 36.057661,
                },
                geometry_rs::Point {
                    x: -114.308624,
                    y: 36.056976,
                },
                geometry_rs::Point {
                    x: -114.3009710000001,
                    y: 36.05746,
                },
                geometry_rs::Point {
                    x: -114.298593,
                    y: 36.057263,
                },
                geometry_rs::Point {
                    x: -114.295941,
                    y: 36.056168,
                },
                geometry_rs::Point {
                    x: -114.2934350000001,
                    y: 36.0545,
                },
                geometry_rs::Point {
                    x: -114.290867,
                    y: 36.050511,
                },
                geometry_rs::Point {
                    x: -114.287992,
                    y: 36.04907,
                },
                geometry_rs::Point {
                    x: -114.284006,
                    y: 36.048242,
                },
                geometry_rs::Point {
                    x: -114.279637,
                    y: 36.046103,
                },
                geometry_rs::Point {
                    x: -114.278166,
                    y: 36.045819,
                },
                geometry_rs::Point {
                    x: -114.273911,
                    y: 36.046529,
                },
                geometry_rs::Point {
                    x: -114.272299,
                    y: 36.046289,
                },
                geometry_rs::Point {
                    x: -114.270862,
                    y: 36.045523,
                },
                geometry_rs::Point {
                    x: -114.269548,
                    y: 36.043769,
                },
                geometry_rs::Point {
                    x: -114.268896,
                    y: 36.04094,
                },
                geometry_rs::Point {
                    x: -114.26922,
                    y: 36.036807,
                },
                geometry_rs::Point {
                    x: -114.268586,
                    y: 36.035034,
                },
                geometry_rs::Point {
                    x: -114.26438,
                    y: 36.027911,
                },
                geometry_rs::Point {
                    x: -114.262388,
                    y: 36.026107,
                },
                geometry_rs::Point {
                    x: -114.259518,
                    y: 36.024206,
                },
                geometry_rs::Point {
                    x: -114.251633,
                    y: 36.019886,
                },
                geometry_rs::Point {
                    x: -114.248419,
                    y: 36.018556,
                },
                geometry_rs::Point {
                    x: -114.246111,
                    y: 36.017164,
                },
                geometry_rs::Point {
                    x: -114.243865,
                    y: 36.015266,
                },
                geometry_rs::Point {
                    x: -114.240439,
                    y: 36.015245,
                },
                geometry_rs::Point {
                    x: -114.238154,
                    y: 36.014473,
                },
                geometry_rs::Point {
                    x: -114.236892,
                    y: 36.013247,
                },
                geometry_rs::Point {
                    x: -114.233443,
                    y: 36.012835,
                },
                geometry_rs::Point {
                    x: -114.231854,
                    y: 36.013147,
                },
                geometry_rs::Point {
                    x: -114.228015,
                    y: 36.014731,
                },
                geometry_rs::Point {
                    x: -114.226459,
                    y: 36.014606,
                },
                geometry_rs::Point {
                    x: -114.224798,
                    y: 36.013699,
                },
                geometry_rs::Point {
                    x: -114.218759,
                    y: 36.014511,
                },
                geometry_rs::Point {
                    x: -114.216609,
                    y: 36.014336,
                },
                geometry_rs::Point {
                    x: -114.214679,
                    y: 36.014806,
                },
                geometry_rs::Point {
                    x: -114.213549,
                    y: 36.014615,
                },
                geometry_rs::Point {
                    x: -114.211932,
                    y: 36.014834,
                },
                geometry_rs::Point {
                    x: -114.206052,
                    y: 36.016634,
                },
                geometry_rs::Point {
                    x: -114.204156,
                    y: 36.016575,
                },
                geometry_rs::Point {
                    x: -114.201227,
                    y: 36.017751,
                },
                geometry_rs::Point {
                    x: -114.200066,
                    y: 36.017743,
                },
                geometry_rs::Point {
                    x: -114.191221,
                    y: 36.020019,
                },
                geometry_rs::Point {
                    x: -114.1858600000001,
                    y: 36.022266,
                },
                geometry_rs::Point {
                    x: -114.179438,
                    y: 36.024313,
                },
                geometry_rs::Point {
                    x: -114.176304,
                    y: 36.026129,
                },
                geometry_rs::Point {
                    x: -114.174683,
                    y: 36.02667,
                },
                geometry_rs::Point {
                    x: -114.164402,
                    y: 36.026852,
                },
                geometry_rs::Point {
                    x: -114.161237,
                    y: 36.026279,
                },
                geometry_rs::Point {
                    x: -114.157344,
                    y: 36.024966,
                },
                geometry_rs::Point {
                    x: -114.1534,
                    y: 36.02317,
                },
                geometry_rs::Point {
                    x: -114.15139,
                    y: 36.023133,
                },
                geometry_rs::Point {
                    x: -114.150225,
                    y: 36.023515,
                },
                geometry_rs::Point {
                    x: -114.145907,
                    y: 36.027229,
                },
                geometry_rs::Point {
                    x: -114.145637,
                    y: 36.028559,
                },
                geometry_rs::Point {
                    x: -114.145672,
                    y: 36.03297,
                },
                geometry_rs::Point {
                    x: -114.144666,
                    y: 36.034272,
                },
                geometry_rs::Point {
                    x: -114.143153,
                    y: 36.035295,
                },
                geometry_rs::Point {
                    x: -114.13826,
                    y: 36.03719,
                },
                geometry_rs::Point {
                    x: -114.137112,
                    y: 36.038491,
                },
                geometry_rs::Point {
                    x: -114.135721,
                    y: 36.041238,
                },
                geometry_rs::Point {
                    x: -114.134841,
                    y: 36.043873,
                },
                geometry_rs::Point {
                    x: -114.134824,
                    y: 36.045343,
                },
                geometry_rs::Point {
                    x: -114.135927,
                    y: 36.050358,
                },
                geometry_rs::Point {
                    x: -114.136206,
                    y: 36.053232,
                },
                geometry_rs::Point {
                    x: -114.1352,
                    y: 36.056946,
                },
                geometry_rs::Point {
                    x: -114.133389,
                    y: 36.061665,
                },
                geometry_rs::Point {
                    x: -114.129768,
                    y: 36.068484,
                },
                geometry_rs::Point {
                    x: -114.125891,
                    y: 36.072935,
                },
                geometry_rs::Point {
                    x: -114.124019,
                    y: 36.075563,
                },
                geometry_rs::Point {
                    x: -114.121186,
                    y: 36.082755,
                },
                geometry_rs::Point {
                    x: -114.119648,
                    y: 36.085822,
                },
                geometry_rs::Point {
                    x: -114.112297,
                    y: 36.09405,
                },
                geometry_rs::Point {
                    x: -114.111998,
                    y: 36.09491,
                },
                geometry_rs::Point {
                    x: -114.1119,
                    y: 36.095845,
                },
                geometry_rs::Point {
                    x: -114.115208,
                    y: 36.099878,
                },
                geometry_rs::Point {
                    x: -114.11707,
                    y: 36.101177,
                },
                geometry_rs::Point {
                    x: -114.119329,
                    y: 36.10193,
                },
                geometry_rs::Point {
                    x: -114.121033,
                    y: 36.103885,
                },
                geometry_rs::Point {
                    x: -114.121779,
                    y: 36.105699,
                },
                geometry_rs::Point {
                    x: -114.12167,
                    y: 36.108294,
                },
                geometry_rs::Point {
                    x: -114.120865,
                    y: 36.11085,
                },
                geometry_rs::Point {
                    x: -114.118497,
                    y: 36.1139,
                },
                geometry_rs::Point {
                    x: -114.116061,
                    y: 36.115471,
                },
                geometry_rs::Point {
                    x: -114.108381,
                    y: 36.119154,
                },
                geometry_rs::Point {
                    x: -114.107419,
                    y: 36.119401,
                },
                geometry_rs::Point {
                    x: -114.100433,
                    y: 36.119359,
                },
                geometry_rs::Point {
                    x: -114.097707,
                    y: 36.120213,
                },
                geometry_rs::Point {
                    x: -114.096994,
                    y: 36.120823,
                },
                geometry_rs::Point {
                    x: -114.092753,
                    y: 36.132356,
                },
                geometry_rs::Point {
                    x: -114.092366,
                    y: 36.135331,
                },
                geometry_rs::Point {
                    x: -114.091701,
                    y: 36.137303,
                },
                geometry_rs::Point {
                    x: -114.089279,
                    y: 36.140326,
                },
                geometry_rs::Point {
                    x: -114.087899,
                    y: 36.142923,
                },
                geometry_rs::Point {
                    x: -114.081234,
                    y: 36.150208,
                },
                geometry_rs::Point {
                    x: -114.07945,
                    y: 36.154625,
                },
                geometry_rs::Point {
                    x: -114.078832,
                    y: 36.157434,
                },
                geometry_rs::Point {
                    x: -114.075641,
                    y: 36.162523,
                },
                geometry_rs::Point {
                    x: -114.071652,
                    y: 36.170921,
                },
                geometry_rs::Point {
                    x: -114.066798,
                    y: 36.179087,
                },
                geometry_rs::Point {
                    x: -114.058662,
                    y: 36.187835,
                },
                geometry_rs::Point {
                    x: -114.052743,
                    y: 36.190919,
                },
                geometry_rs::Point {
                    x: -114.049484,
                    y: 36.192134,
                },
                geometry_rs::Point {
                    x: -114.043944,
                    y: 36.19335,
                },
                geometry_rs::Point {
                    x: -114.043849,
                    y: 36.245114,
                },
                geometry_rs::Point {
                    x: -114.045518,
                    y: 36.27439,
                },
                geometry_rs::Point {
                    x: -114.045559,
                    y: 36.288837,
                },
                geometry_rs::Point {
                    x: -114.045033,
                    y: 36.30305,
                },
                geometry_rs::Point {
                    x: -114.044345,
                    y: 36.310234,
                },
                geometry_rs::Point {
                    x: -114.044051,
                    y: 36.317628,
                },
                geometry_rs::Point {
                    x: -114.044776,
                    y: 36.331969,
                },
                geometry_rs::Point {
                    x: -114.044702,
                    y: 36.346298,
                },
                geometry_rs::Point {
                    x: -114.043034,
                    y: 36.38587,
                },
                geometry_rs::Point {
                    x: -114.0428430000001,
                    y: 36.448175,
                },
                geometry_rs::Point {
                    x: -114.043133,
                    y: 36.469716,
                },
                geometry_rs::Point {
                    x: -114.044816,
                    y: 36.491343,
                },
                geometry_rs::Point {
                    x: -114.045647,
                    y: 36.521095,
                },
                geometry_rs::Point {
                    x: -114.04632,
                    y: 36.564615,
                },
                geometry_rs::Point {
                    x: -114.049935,
                    y: 36.709521,
                },
                geometry_rs::Point {
                    x: -114.049973,
                    y: 36.738672,
                },
                geometry_rs::Point {
                    x: -114.050327,
                    y: 36.752899,
                },
                geometry_rs::Point {
                    x: -114.049879,
                    y: 36.781909,
                },
                geometry_rs::Point {
                    x: -114.050502,
                    y: 36.895232,
                },
                geometry_rs::Point {
                    x: -114.049995,
                    y: 36.957769,
                },
                geometry_rs::Point {
                    x: -114.0506000000001,
                    y: 37.000396,
                },
                geometry_rs::Point {
                    x: -114.0008,
                    y: 37.000448,
                },
                geometry_rs::Point {
                    x: -113.96266,
                    y: 36.999973,
                },
                geometry_rs::Point {
                    x: -113.052912,
                    y: 36.999983,
                },
                geometry_rs::Point {
                    x: -112.875756,
                    y: 37.000533,
                },
                geometry_rs::Point {
                    x: -112.538546,
                    y: 37.000652,
                },
                geometry_rs::Point {
                    x: -112.529846,
                    y: 37.000899,
                },
                geometry_rs::Point {
                    x: -112.36102,
                    y: 37.001114,
                },
                geometry_rs::Point {
                    x: -112.36037,
                    y: 37.000912,
                },
                geometry_rs::Point {
                    x: -112.359329,
                    y: 37.001117,
                },
                geometry_rs::Point {
                    x: -112.125741,
                    y: 37.001237,
                },
                geometry_rs::Point {
                    x: -112.000735,
                    y: 37.000959,
                },
                geometry_rs::Point {
                    x: -111.62572,
                    y: 37.001401,
                },
                geometry_rs::Point {
                    x: -111.616249,
                    y: 37.001647,
                },
                geometry_rs::Point {
                    x: -111.406146,
                    y: 37.001481,
                },
                geometry_rs::Point {
                    x: -111.405895,
                    y: 37.001702,
                },
                geometry_rs::Point {
                    x: -111.313211,
                    y: 37.000894,
                },
                geometry_rs::Point {
                    x: -111.312169,
                    y: 37.001193,
                },
                geometry_rs::Point {
                    x: -111.305843,
                    y: 37.000776,
                },
                geometry_rs::Point {
                    x: -111.278221,
                    y: 37.000467,
                },
                geometry_rs::Point {
                    x: -111.254853,
                    y: 37.001076,
                },
                geometry_rs::Point {
                    x: -111.133718,
                    y: 37.000779,
                },
                geometry_rs::Point {
                    x: -111.081493,
                    y: 37.002261,
                },
                geometry_rs::Point {
                    x: -111.052354,
                    y: 37.00246,
                },
                geometry_rs::Point {
                    x: -111.00182,
                    y: 37.002293,
                },
                geometry_rs::Point {
                    x: -110.625691,
                    y: 37.003725,
                },
                geometry_rs::Point {
                    x: -110.625605,
                    y: 37.003416,
                },
                geometry_rs::Point {
                    x: -110.599512,
                    y: 37.003448,
                },
                geometry_rs::Point {
                    x: -110.509004,
                    y: 37.003985,
                },
                geometry_rs::Point {
                    x: -110.50069,
                    y: 37.00426,
                },
                geometry_rs::Point {
                    x: -110.490908,
                    y: 37.003566,
                },
                geometry_rs::Point {
                    x: -110.478446,
                    y: 36.999996,
                },
                geometry_rs::Point {
                    x: -110.47729,
                    y: 36.999997,
                },
                geometry_rs::Point {
                    x: -110.47019,
                    y: 36.997997,
                },
                geometry_rs::Point {
                    x: -110.023043,
                    y: 36.998601,
                },
                geometry_rs::Point {
                    x: -110.000876,
                    y: 36.998502,
                },
                geometry_rs::Point {
                    x: -110.000677,
                    y: 36.997968,
                },
                geometry_rs::Point {
                    x: -109.969958,
                    y: 36.997949,
                },
                geometry_rs::Point {
                    x: -109.938511,
                    y: 36.998491,
                },
                geometry_rs::Point {
                    x: -109.750669,
                    y: 36.99816,
                },
                geometry_rs::Point {
                    x: -109.743284,
                    y: 36.998453,
                },
                geometry_rs::Point {
                    x: -109.625658,
                    y: 36.998308,
                },
                geometry_rs::Point {
                    x: -109.495338,
                    y: 36.999105,
                },
                geometry_rs::Point {
                    x: -109.362565,
                    y: 36.999304,
                },
                geometry_rs::Point {
                    x: -109.125691,
                    y: 36.999389,
                },
                geometry_rs::Point {
                    x: -109.045223,
                    y: 36.999084,
                },
                geometry_rs::Point {
                    x: -109.045554,
                    y: 36.645013,
                },
                geometry_rs::Point {
                    x: -109.04539,
                    y: 36.503241,
                },
                geometry_rs::Point {
                    x: -109.045946,
                    y: 36.375002,
                },
                geometry_rs::Point {
                    x: -109.045637,
                    y: 36.374625,
                },
                geometry_rs::Point {
                    x: -109.045744,
                    y: 36.257214,
                },
                geometry_rs::Point {
                    x: -109.046024,
                    y: 36.247197,
                },
                geometry_rs::Point {
                    x: -109.045877,
                    y: 36.188719,
                },
                geometry_rs::Point {
                    x: -109.046183,
                    y: 36.181751,
                },
                geometry_rs::Point {
                    x: -109.045726,
                    y: 36.116908,
                },
                geometry_rs::Point {
                    x: -109.045767,
                    y: 36.033679,
                },
                geometry_rs::Point {
                    x: -109.046124,
                    y: 35.990618,
                },
                geometry_rs::Point {
                    x: -109.046009,
                    y: 35.875012,
                },
                geometry_rs::Point {
                    x: -109.046423,
                    y: 35.624911,
                },
                geometry_rs::Point {
                    x: -109.046181,
                    y: 35.614569,
                },
                geometry_rs::Point {
                    x: -109.046795,
                    y: 35.379918,
                },
                geometry_rs::Point {
                    x: -109.046084,
                    y: 35.249986,
                },
                geometry_rs::Point {
                    x: -109.046256,
                    y: 35.125041,
                },
                geometry_rs::Point {
                    x: -109.045842,
                    y: 34.966076,
                },
                geometry_rs::Point {
                    x: -109.046136,
                    y: 34.875006,
                },
                geometry_rs::Point {
                    x: -109.046072,
                    y: 34.828566,
                },
                geometry_rs::Point {
                    x: -109.045626,
                    y: 34.814226,
                },
                geometry_rs::Point {
                    x: -109.046104,
                    y: 34.799981,
                },
                geometry_rs::Point {
                    x: -109.045363,
                    y: 34.785406,
                },
                geometry_rs::Point {
                    x: -109.046087,
                    y: 34.770963,
                },
                geometry_rs::Point {
                    x: -109.046175,
                    y: 34.520102,
                },
                geometry_rs::Point {
                    x: -109.046561,
                    y: 34.379479,
                },
                geometry_rs::Point {
                    x: -109.046337,
                    y: 34.283639,
                },
                geometry_rs::Point {
                    x: -109.046664,
                    y: 34.250046,
                },
                geometry_rs::Point {
                    x: -109.04696,
                    y: 34.068968,
                },
                geometry_rs::Point {
                    x: -109.047006,
                    y: 34.00005,
                },
                geometry_rs::Point {
                    x: -109.046426,
                    y: 33.875052,
                },
                geometry_rs::Point {
                    x: -109.046869,
                    y: 33.844183,
                },
                geometry_rs::Point {
                    x: -109.047145,
                    y: 33.74001,
                },
                geometry_rs::Point {
                    x: -109.046662,
                    y: 33.625055,
                },
                geometry_rs::Point {
                    x: -109.046825,
                    y: 33.469389,
                },
                geometry_rs::Point {
                    x: -109.047309,
                    y: 33.462131,
                },
                geometry_rs::Point {
                    x: -109.046928,
                    y: 33.4428,
                },
                geometry_rs::Point {
                    x: -109.047304,
                    y: 33.439442,
                },
                geometry_rs::Point {
                    x: -109.047298,
                    y: 33.409774,
                },
                geometry_rs::Point {
                    x: -109.046564,
                    y: 33.375059,
                },
                geometry_rs::Point {
                    x: -109.047045,
                    y: 33.36928,
                },
                geometry_rs::Point {
                    x: -109.046827,
                    y: 33.365271,
                },
                geometry_rs::Point {
                    x: -109.047104,
                    y: 33.27046,
                },
                geometry_rs::Point {
                    x: -109.04747,
                    y: 33.250168,
                },
                geometry_rs::Point {
                    x: -109.047122,
                    y: 33.2408,
                },
                geometry_rs::Point {
                    x: -109.047324,
                    y: 33.18408,
                },
                geometry_rs::Point {
                    x: -109.047208,
                    y: 33.107377,
                },
                geometry_rs::Point {
                    x: -109.046905,
                    y: 33.091931,
                },
                geometry_rs::Point {
                    x: -109.047513,
                    y: 33.059137,
                },
                geometry_rs::Point {
                    x: -109.047382,
                    y: 33.000311,
                },
                geometry_rs::Point {
                    x: -109.04711,
                    y: 32.99225,
                },
                geometry_rs::Point {
                    x: -109.047117,
                    y: 32.777569,
                },
                geometry_rs::Point {
                    x: -109.047518,
                    y: 32.749997,
                },
                geometry_rs::Point {
                    x: -109.047796,
                    y: 32.68263,
                },
                geometry_rs::Point {
                    x: -109.047912,
                    y: 32.500261,
                },
                geometry_rs::Point {
                    x: -109.047629,
                    y: 32.413987,
                },
                geometry_rs::Point {
                    x: -109.048323,
                    y: 32.070887,
                },
                geometry_rs::Point {
                    x: -109.048731,
                    y: 32.028174,
                },
                geometry_rs::Point {
                    x: -109.048465,
                    y: 32.000089,
                },
                geometry_rs::Point {
                    x: -109.048738,
                    y: 31.876905,
                },
                geometry_rs::Point {
                    x: -109.049048,
                    y: 31.870689,
                },
                geometry_rs::Point {
                    x: -109.049298,
                    y: 31.796742,
                },
                geometry_rs::Point {
                    x: -109.04899,
                    y: 31.721922,
                },
                geometry_rs::Point {
                    x: -109.049311,
                    y: 31.544932,
                },
                geometry_rs::Point {
                    x: -109.050173,
                    y: 31.480004,
                },
                geometry_rs::Point {
                    x: -109.049934,
                    y: 31.437907,
                },
                geometry_rs::Point {
                    x: -109.050044,
                    y: 31.332502,
                },
                geometry_rs::Point {
                    x: -109.1256,
                    y: 31.332685,
                },
                geometry_rs::Point {
                    x: -109.271744,
                    y: 31.333942,
                },
                geometry_rs::Point {
                    x: -109.49449,
                    y: 31.334125,
                },
                geometry_rs::Point {
                    x: -109.500621,
                    y: 31.333911,
                },
                geometry_rs::Point {
                    x: -109.875628,
                    y: 31.33405,
                },
                geometry_rs::Point {
                    x: -110.000613,
                    y: 31.333145,
                },
                geometry_rs::Point {
                    x: -110.140512,
                    y: 31.333965,
                },
                geometry_rs::Point {
                    x: -110.375635,
                    y: 31.332896,
                },
                geometry_rs::Point {
                    x: -110.460172,
                    y: 31.332827,
                },
                geometry_rs::Point {
                    x: -110.68143,
                    y: 31.33309,
                },
                geometry_rs::Point {
                    x: -110.750638,
                    y: 31.333636,
                },
                geometry_rs::Point {
                    x: -110.795467,
                    y: 31.33363,
                },
                geometry_rs::Point {
                    x: -110.94232,
                    y: 31.332833,
                },
                geometry_rs::Point {
                    x: -111.000643,
                    y: 31.332177,
                },
                geometry_rs::Point {
                    x: -111.074825,
                    y: 31.332239,
                },
                geometry_rs::Point {
                    x: -111.125646,
                    y: 31.348978,
                },
                geometry_rs::Point {
                    x: -111.129451,
                    y: 31.349979,
                },
                geometry_rs::Point {
                    x: -111.357436,
                    y: 31.423346,
                },
                geometry_rs::Point {
                    x: -111.500659,
                    y: 31.468862,
                },
                geometry_rs::Point {
                    x: -111.560194,
                    y: 31.488138,
                },
                geometry_rs::Point {
                    x: -111.659998,
                    y: 31.519448,
                },
                geometry_rs::Point {
                    x: -111.738873,
                    y: 31.544718,
                },
                geometry_rs::Point {
                    x: -111.875674,
                    y: 31.587657,
                },
                geometry_rs::Point {
                    x: -111.979304,
                    y: 31.620648,
                },
                geometry_rs::Point {
                    x: -112.200717,
                    y: 31.690033,
                },
                geometry_rs::Point {
                    x: -112.365328,
                    y: 31.741078,
                },
                geometry_rs::Point {
                    x: -112.375759,
                    y: 31.743987,
                },
                geometry_rs::Point {
                    x: -112.399254,
                    y: 31.751638,
                },
                geometry_rs::Point {
                    x: -112.433246,
                    y: 31.762162,
                },
                geometry_rs::Point {
                    x: -112.737399,
                    y: 31.855527,
                },
                geometry_rs::Point {
                    x: -112.800213,
                    y: 31.87507,
                },
                geometry_rs::Point {
                    x: -112.834233,
                    y: 31.885137,
                },
                geometry_rs::Point {
                    x: -112.871505,
                    y: 31.896838,
                },
                geometry_rs::Point {
                    x: -113.125961,
                    y: 31.97278,
                },
                geometry_rs::Point {
                    x: -113.21163,
                    y: 32.000061,
                },
                geometry_rs::Point {
                    x: -113.211365,
                    y: 32.000061,
                },
                geometry_rs::Point {
                    x: -113.217307,
                    y: 32.002106,
                },
                geometry_rs::Point {
                    x: -113.250731,
                    y: 32.012405,
                },
                geometry_rs::Point {
                    x: -113.493196,
                    y: 32.088943,
                },
                geometry_rs::Point {
                    x: -113.750756,
                    y: 32.169005,
                },
                geometry_rs::Point {
                    x: -113.78168,
                    y: 32.179034,
                },
                geometry_rs::Point {
                    x: -114.250775,
                    y: 32.323909,
                },
                geometry_rs::Point {
                    x: -114.625785,
                    y: 32.43789,
                },
                geometry_rs::Point {
                    x: -114.790245,
                    y: 32.487505,
                },
                geometry_rs::Point {
                    x: -114.813613,
                    y: 32.494276,
                },
                geometry_rs::Point {
                    x: -114.8139910000001,
                    y: 32.497231,
                },
                geometry_rs::Point {
                    x: -114.812316,
                    y: 32.500054,
                },
                geometry_rs::Point {
                    x: -114.813402,
                    y: 32.501764,
                },
                geometry_rs::Point {
                    x: -114.8137530000001,
                    y: 32.50426,
                },
                geometry_rs::Point {
                    x: -114.815185,
                    y: 32.506023,
                },
                geometry_rs::Point {
                    x: -114.81651,
                    y: 32.506963,
                },
                geometry_rs::Point {
                    x: -114.8165910000001,
                    y: 32.507696,
                },
                geometry_rs::Point {
                    x: -114.815591,
                    y: 32.508612,
                },
                geometry_rs::Point {
                    x: -114.8143210000001,
                    y: 32.509023,
                },
                geometry_rs::Point {
                    x: -114.812942,
                    y: 32.509116,
                },
                geometry_rs::Point {
                    x: -114.810159,
                    y: 32.508383,
                },
                geometry_rs::Point {
                    x: -114.807726,
                    y: 32.508726,
                },
                geometry_rs::Point {
                    x: -114.804076,
                    y: 32.510375,
                },
                geometry_rs::Point {
                    x: -114.802833,
                    y: 32.511749,
                },
                geometry_rs::Point {
                    x: -114.802211,
                    y: 32.513191,
                },
                geometry_rs::Point {
                    x: -114.802238,
                    y: 32.515206,
                },
                geometry_rs::Point {
                    x: -114.80367,
                    y: 32.516374,
                },
                geometry_rs::Point {
                    x: -114.807753,
                    y: 32.516925,
                },
                geometry_rs::Point {
                    x: -114.809672,
                    y: 32.517567,
                },
                geometry_rs::Point {
                    x: -114.810374,
                    y: 32.518391,
                },
                geometry_rs::Point {
                    x: -114.809969,
                    y: 32.520291,
                },
                geometry_rs::Point {
                    x: -114.810482,
                    y: 32.521758,
                },
                geometry_rs::Point {
                    x: -114.810969,
                    y: 32.522444,
                },
                geometry_rs::Point {
                    x: -114.812888,
                    y: 32.52359,
                },
                geometry_rs::Point {
                    x: -114.8133480000001,
                    y: 32.524186,
                },
                geometry_rs::Point {
                    x: -114.812645,
                    y: 32.525399,
                },
                geometry_rs::Point {
                    x: -114.811293,
                    y: 32.526429,
                },
                geometry_rs::Point {
                    x: -114.810563,
                    y: 32.527666,
                },
                geometry_rs::Point {
                    x: -114.808617,
                    y: 32.529017,
                },
                geometry_rs::Point {
                    x: -114.80640000000011,
                    y: 32.531191,
                },
                geometry_rs::Point {
                    x: -114.804858,
                    y: 32.533689,
                },
                geometry_rs::Point {
                    x: -114.802559,
                    y: 32.535521,
                },
                geometry_rs::Point {
                    x: -114.802181,
                    y: 32.536414,
                },
                geometry_rs::Point {
                    x: -114.802018,
                    y: 32.53946,
                },
                geometry_rs::Point {
                    x: -114.80237,
                    y: 32.540078,
                },
                geometry_rs::Point {
                    x: -114.8047760000001,
                    y: 32.541659,
                },
                geometry_rs::Point {
                    x: -114.805966,
                    y: 32.545346,
                },
                geometry_rs::Point {
                    x: -114.8058300000001,
                    y: 32.546354,
                },
                geometry_rs::Point {
                    x: -114.803883,
                    y: 32.548001,
                },
                geometry_rs::Point {
                    x: -114.795635,
                    y: 32.550956,
                },
                geometry_rs::Point {
                    x: -114.793769,
                    y: 32.552329,
                },
                geometry_rs::Point {
                    x: -114.792065,
                    y: 32.555009,
                },
                geometry_rs::Point {
                    x: -114.791551,
                    y: 32.557023,
                },
                geometry_rs::Point {
                    x: -114.791523,
                    y: 32.558602,
                },
                geometry_rs::Point {
                    x: -114.7929550000001,
                    y: 32.562085,
                },
                geometry_rs::Point {
                    x: -114.792088,
                    y: 32.568497,
                },
                geometry_rs::Point {
                    x: -114.7923580000001,
                    y: 32.569091,
                },
                geometry_rs::Point {
                    x: -114.793224,
                    y: 32.569459,
                },
                geometry_rs::Point {
                    x: -114.794684,
                    y: 32.568703,
                },
                geometry_rs::Point {
                    x: -114.795253,
                    y: 32.56662,
                },
                geometry_rs::Point {
                    x: -114.79766,
                    y: 32.564444,
                },
                geometry_rs::Point {
                    x: -114.801311,
                    y: 32.562865,
                },
                geometry_rs::Point {
                    x: -114.803664,
                    y: 32.560689,
                },
                geometry_rs::Point {
                    x: -114.8068300000001,
                    y: 32.55888,
                },
                geometry_rs::Point {
                    x: -114.808885,
                    y: 32.558467,
                },
                geometry_rs::Point {
                    x: -114.810318,
                    y: 32.558628,
                },
                geometry_rs::Point {
                    x: -114.812914,
                    y: 32.560049,
                },
                geometry_rs::Point {
                    x: -114.813995,
                    y: 32.562201,
                },
                geometry_rs::Point {
                    x: -114.814212,
                    y: 32.56369,
                },
                geometry_rs::Point {
                    x: -114.813968,
                    y: 32.566209,
                },
                geometry_rs::Point {
                    x: -114.812995,
                    y: 32.568706,
                },
                geometry_rs::Point {
                    x: -114.81148,
                    y: 32.569781,
                },
                geometry_rs::Point {
                    x: -114.804421,
                    y: 32.572941,
                },
                geometry_rs::Point {
                    x: -114.803474,
                    y: 32.573628,
                },
                geometry_rs::Point {
                    x: -114.801877,
                    y: 32.576009,
                },
                geometry_rs::Point {
                    x: -114.801471,
                    y: 32.578255,
                },
                geometry_rs::Point {
                    x: -114.80193,
                    y: 32.579194,
                },
                geometry_rs::Point {
                    x: -114.803879,
                    y: 32.580889,
                },
                geometry_rs::Point {
                    x: -114.803987,
                    y: 32.582652,
                },
                geometry_rs::Point {
                    x: -114.802823,
                    y: 32.585079,
                },
                geometry_rs::Point {
                    x: -114.800441,
                    y: 32.588079,
                },
                geometry_rs::Point {
                    x: -114.799737,
                    y: 32.592177,
                },
                geometry_rs::Point {
                    x: -114.799683,
                    y: 32.593621,
                },
                geometry_rs::Point {
                    x: -114.801251,
                    y: 32.596232,
                },
                geometry_rs::Point {
                    x: -114.801548,
                    y: 32.598591,
                },
                geometry_rs::Point {
                    x: -114.802361,
                    y: 32.59937,
                },
                geometry_rs::Point {
                    x: -114.805932,
                    y: 32.600721,
                },
                geometry_rs::Point {
                    x: -114.8069050000001,
                    y: 32.60143,
                },
                geometry_rs::Point {
                    x: -114.808041,
                    y: 32.603172,
                },
                geometry_rs::Point {
                    x: -114.807879,
                    y: 32.605416,
                },
                geometry_rs::Point {
                    x: -114.809042,
                    y: 32.608806,
                },
                geometry_rs::Point {
                    x: -114.808906,
                    y: 32.612951,
                },
                geometry_rs::Point {
                    x: -114.809555,
                    y: 32.616203,
                },
                geometry_rs::Point {
                    x: -114.808662,
                    y: 32.619157,
                },
                geometry_rs::Point {
                    x: -114.80739,
                    y: 32.621332,
                },
                geometry_rs::Point {
                    x: -114.806821,
                    y: 32.621721,
                },
                geometry_rs::Point {
                    x: -114.799302,
                    y: 32.625115,
                },
                geometry_rs::Point {
                    x: -114.797564,
                    y: 32.624578,
                },
                geometry_rs::Point {
                    x: -114.794102,
                    y: 32.622475,
                },
                geometry_rs::Point {
                    x: -114.7926400000001,
                    y: 32.621948,
                },
                geometry_rs::Point {
                    x: -114.791179,
                    y: 32.621833,
                },
                geometry_rs::Point {
                    x: -114.787715,
                    y: 32.623573,
                },
                geometry_rs::Point {
                    x: -114.782573,
                    y: 32.624304,
                },
                geometry_rs::Point {
                    x: -114.781896,
                    y: 32.624702,
                },
                geometry_rs::Point {
                    x: -114.781766,
                    y: 32.625613,
                },
                geometry_rs::Point {
                    x: -114.782518,
                    y: 32.628625,
                },
                geometry_rs::Point {
                    x: -114.782235,
                    y: 32.630215,
                },
                geometry_rs::Point {
                    x: -114.779215,
                    y: 32.633578,
                },
                geometry_rs::Point {
                    x: -114.77457,
                    y: 32.63593,
                },
                geometry_rs::Point {
                    x: -114.771978,
                    y: 32.637954,
                },
                geometry_rs::Point {
                    x: -114.768199,
                    y: 32.639874,
                },
                geometry_rs::Point {
                    x: -114.764382,
                    y: 32.642666,
                },
                geometry_rs::Point {
                    x: -114.76331,
                    y: 32.644616,
                },
                geometry_rs::Point {
                    x: -114.763512,
                    y: 32.645995,
                },
                geometry_rs::Point {
                    x: -114.764917,
                    y: 32.648079,
                },
                geometry_rs::Point {
                    x: -114.76495,
                    y: 32.649391,
                },
                geometry_rs::Point {
                    x: -114.75831,
                    y: 32.655178,
                },
                geometry_rs::Point {
                    x: -114.751079,
                    y: 32.659789,
                },
                geometry_rs::Point {
                    x: -114.7494800000001,
                    y: 32.66178,
                },
                geometry_rs::Point {
                    x: -114.7480000000001,
                    y: 32.664184,
                },
                geometry_rs::Point {
                    x: -114.748183,
                    y: 32.665098,
                },
                geometry_rs::Point {
                    x: -114.747817,
                    y: 32.667777,
                },
                geometry_rs::Point {
                    x: -114.746383,
                    y: 32.669853,
                },
                geometry_rs::Point {
                    x: -114.745344,
                    y: 32.67219,
                },
                geometry_rs::Point {
                    x: -114.7449,
                    y: 32.677231,
                },
                geometry_rs::Point {
                    x: -114.744349,
                    y: 32.678935,
                },
                geometry_rs::Point {
                    x: -114.740541,
                    y: 32.684196,
                },
                geometry_rs::Point {
                    x: -114.739405,
                    y: 32.686385,
                },
                geometry_rs::Point {
                    x: -114.730453,
                    y: 32.698844,
                },
                geometry_rs::Point {
                    x: -114.72981,
                    y: 32.700282,
                },
                geometry_rs::Point {
                    x: -114.72974,
                    y: 32.703121,
                },
                geometry_rs::Point {
                    x: -114.730086,
                    y: 32.704298,
                },
                geometry_rs::Point {
                    x: -114.728408,
                    y: 32.706648,
                },
                geometry_rs::Point {
                    x: -114.726974,
                    y: 32.707875,
                },
                geometry_rs::Point {
                    x: -114.72534,
                    y: 32.710369,
                },
                geometry_rs::Point {
                    x: -114.72241,
                    y: 32.713597,
                },
                geometry_rs::Point {
                    x: -114.719938,
                    y: 32.71829,
                },
                geometry_rs::Point {
                    x: -114.717695,
                    y: 32.721547,
                },
                geometry_rs::Point {
                    x: -114.715788,
                    y: 32.727758,
                },
                geometry_rs::Point {
                    x: -114.714522,
                    y: 32.73039,
                },
                geometry_rs::Point {
                    x: -114.712629,
                    y: 32.732678,
                },
                geometry_rs::Point {
                    x: -114.710615,
                    y: 32.733936,
                },
                geometry_rs::Point {
                    x: -114.709074,
                    y: 32.735456,
                },
                geometry_rs::Point {
                    x: -114.706114,
                    y: 32.740986,
                },
                geometry_rs::Point {
                    x: -114.70294,
                    y: 32.744793,
                },
                geometry_rs::Point {
                    x: -114.7015820000001,
                    y: 32.745632,
                },
                geometry_rs::Point {
                    x: -114.699247,
                    y: 32.745098,
                },
                geometry_rs::Point {
                    x: -114.695387,
                    y: 32.742244,
                },
                geometry_rs::Point {
                    x: -114.691801,
                    y: 32.740147,
                },
                geometry_rs::Point {
                    x: -114.689282,
                    y: 32.737927,
                },
                geometry_rs::Point {
                    x: -114.6882300000001,
                    y: 32.73753,
                },
                geometry_rs::Point {
                    x: -114.682614,
                    y: 32.737348,
                },
                geometry_rs::Point {
                    x: -114.672025,
                    y: 32.734951,
                },
                geometry_rs::Point {
                    x: -114.665921,
                    y: 32.734028,
                },
                geometry_rs::Point {
                    x: -114.654247,
                    y: 32.73357,
                },
                geometry_rs::Point {
                    x: -114.645353,
                    y: 32.732139,
                },
                geometry_rs::Point {
                    x: -114.6350060000001,
                    y: 32.731372,
                },
                geometry_rs::Point {
                    x: -114.629299,
                    y: 32.729908,
                },
                geometry_rs::Point {
                    x: -114.617479,
                    y: 32.728243,
                },
                geometry_rs::Point {
                    x: -114.61567,
                    y: 32.728454,
                },
                geometry_rs::Point {
                    x: -114.61587,
                    y: 32.729717,
                },
                geometry_rs::Point {
                    x: -114.615501,
                    y: 32.730044,
                },
                geometry_rs::Point {
                    x: -114.615504,
                    y: 32.731449,
                },
                geometry_rs::Point {
                    x: -114.614786,
                    y: 32.732846,
                },
                geometry_rs::Point {
                    x: -114.6147870000001,
                    y: 32.734076,
                },
                geometry_rs::Point {
                    x: -114.615112,
                    y: 32.734515,
                },
                geometry_rs::Point {
                    x: -114.581784,
                    y: 32.734946,
                },
                geometry_rs::Point {
                    x: -114.581736,
                    y: 32.74232,
                },
                geometry_rs::Point {
                    x: -114.564508,
                    y: 32.742274,
                },
                geometry_rs::Point {
                    x: -114.564447,
                    y: 32.749554,
                },
                geometry_rs::Point {
                    x: -114.539224,
                    y: 32.749812,
                },
                geometry_rs::Point {
                    x: -114.539092,
                    y: 32.756949,
                },
                geometry_rs::Point {
                    x: -114.526856,
                    y: 32.757094,
                },
                geometry_rs::Point {
                    x: -114.528443,
                    y: 32.767276,
                },
                geometry_rs::Point {
                    x: -114.529264,
                    y: 32.769484,
                },
                geometry_rs::Point {
                    x: -114.531831,
                    y: 32.774264,
                },
                geometry_rs::Point {
                    x: -114.532432,
                    y: 32.776922,
                },
                geometry_rs::Point {
                    x: -114.532426,
                    y: 32.778644,
                },
                geometry_rs::Point {
                    x: -114.531746,
                    y: 32.782503,
                },
                geometry_rs::Point {
                    x: -114.531669,
                    y: 32.791185,
                },
                geometry_rs::Point {
                    x: -114.5296330000001,
                    y: 32.795477,
                },
                geometry_rs::Point {
                    x: -114.522031,
                    y: 32.801675,
                },
                geometry_rs::Point {
                    x: -114.520385,
                    y: 32.803576,
                },
                geometry_rs::Point {
                    x: -114.5203630000001,
                    y: 32.804385,
                },
                geometry_rs::Point {
                    x: -114.519758,
                    y: 32.805676,
                },
                geometry_rs::Point {
                    x: -114.515389,
                    y: 32.811439,
                },
                geometry_rs::Point {
                    x: -114.510327,
                    y: 32.816488,
                },
                geometry_rs::Point {
                    x: -114.494116,
                    y: 32.823287,
                },
                geometry_rs::Point {
                    x: -114.475892,
                    y: 32.838693,
                },
                geometry_rs::Point {
                    x: -114.468971,
                    y: 32.845155,
                },
                geometry_rs::Point {
                    x: -114.465711,
                    y: 32.873681,
                },
                geometry_rs::Point {
                    x: -114.465172,
                    y: 32.885295,
                },
                geometry_rs::Point {
                    x: -114.463307,
                    y: 32.899116,
                },
                geometry_rs::Point {
                    x: -114.462929,
                    y: 32.907944,
                },
                geometry_rs::Point {
                    x: -114.46365,
                    y: 32.911682,
                },
                geometry_rs::Point {
                    x: -114.464448,
                    y: 32.913128,
                },
                geometry_rs::Point {
                    x: -114.473713,
                    y: 32.920594,
                },
                geometry_rs::Point {
                    x: -114.47664,
                    y: 32.923628,
                },
                geometry_rs::Point {
                    x: -114.4779520000001,
                    y: 32.925706,
                },
                geometry_rs::Point {
                    x: -114.479005,
                    y: 32.928291,
                },
                geometry_rs::Point {
                    x: -114.480783,
                    y: 32.933678,
                },
                geometry_rs::Point {
                    x: -114.480925,
                    y: 32.936276,
                },
                geometry_rs::Point {
                    x: -114.48074,
                    y: 32.937027,
                },
                geometry_rs::Point {
                    x: -114.478456,
                    y: 32.940555,
                },
                geometry_rs::Point {
                    x: -114.474042,
                    y: 32.94515,
                },
                geometry_rs::Point {
                    x: -114.470768,
                    y: 32.949424,
                },
                geometry_rs::Point {
                    x: -114.468536,
                    y: 32.953922,
                },
                geometry_rs::Point {
                    x: -114.467624,
                    y: 32.956663,
                },
                geometry_rs::Point {
                    x: -114.467274,
                    y: 32.960172,
                },
                geometry_rs::Point {
                    x: -114.467367,
                    y: 32.965384,
                },
                geometry_rs::Point {
                    x: -114.468379,
                    y: 32.970745,
                },
                geometry_rs::Point {
                    x: -114.468995,
                    y: 32.972239,
                },
                geometry_rs::Point {
                    x: -114.470511,
                    y: 32.973858,
                },
                geometry_rs::Point {
                    x: -114.472606,
                    y: 32.974654,
                },
                geometry_rs::Point {
                    x: -114.475171,
                    y: 32.975154,
                },
                geometry_rs::Point {
                    x: -114.477308,
                    y: 32.975023,
                },
                geometry_rs::Point {
                    x: -114.479477,
                    y: 32.974189,
                },
                geometry_rs::Point {
                    x: -114.480831,
                    y: 32.973362,
                },
                geometry_rs::Point {
                    x: -114.481315,
                    y: 32.972064,
                },
                geometry_rs::Point {
                    x: -114.484806,
                    y: 32.971339,
                },
                geometry_rs::Point {
                    x: -114.488625,
                    y: 32.969946,
                },
                geometry_rs::Point {
                    x: -114.490129,
                    y: 32.969884,
                },
                geometry_rs::Point {
                    x: -114.492184,
                    y: 32.971021,
                },
                geometry_rs::Point {
                    x: -114.492938,
                    y: 32.971781,
                },
                geometry_rs::Point {
                    x: -114.494212,
                    y: 32.974262,
                },
                geometry_rs::Point {
                    x: -114.495712,
                    y: 32.980075,
                },
                geometry_rs::Point {
                    x: -114.496798,
                    y: 32.986534,
                },
                geometry_rs::Point {
                    x: -114.497052,
                    y: 32.990206,
                },
                geometry_rs::Point {
                    x: -114.49941,
                    y: 33.00004,
                },
                geometry_rs::Point {
                    x: -114.499797,
                    y: 33.003905,
                },
                geometry_rs::Point {
                    x: -114.50287,
                    y: 33.011154,
                },
                geometry_rs::Point {
                    x: -114.506129,
                    y: 33.017009,
                },
                geometry_rs::Point {
                    x: -114.507956,
                    y: 33.019708,
                },
                geometry_rs::Point {
                    x: -114.511343,
                    y: 33.023455,
                },
                geometry_rs::Point {
                    x: -114.5149,
                    y: 33.026524,
                },
                geometry_rs::Point {
                    x: -114.52013,
                    y: 33.029984,
                },
                geometry_rs::Point {
                    x: -114.523578,
                    y: 33.03096,
                },
                geometry_rs::Point {
                    x: -114.538459,
                    y: 33.033422,
                },
                geometry_rs::Point {
                    x: -114.553189,
                    y: 33.033974,
                },
                geometry_rs::Point {
                    x: -114.56085,
                    y: 33.035285,
                },
                geometry_rs::Point {
                    x: -114.5648,
                    y: 33.035077,
                },
                geometry_rs::Point {
                    x: -114.571653,
                    y: 33.036624,
                },
                geometry_rs::Point {
                    x: -114.575161,
                    y: 33.036541,
                },
                geometry_rs::Point {
                    x: -114.578287,
                    y: 33.035375,
                },
                geometry_rs::Point {
                    x: -114.581404,
                    y: 33.032545,
                },
                geometry_rs::Point {
                    x: -114.584765,
                    y: 33.02823,
                },
                geometry_rs::Point {
                    x: -114.586982,
                    y: 33.026944,
                },
                geometry_rs::Point {
                    x: -114.589778,
                    y: 33.026228,
                },
                geometry_rs::Point {
                    x: -114.598093,
                    y: 33.025384,
                },
                geometry_rs::Point {
                    x: -114.601014,
                    y: 33.02541,
                },
                geometry_rs::Point {
                    x: -114.611584,
                    y: 33.026221,
                },
                geometry_rs::Point {
                    x: -114.618788,
                    y: 33.027202,
                },
                geometry_rs::Point {
                    x: -114.625787,
                    y: 33.029435,
                },
                geometry_rs::Point {
                    x: -114.628294,
                    y: 33.03105,
                },
                geometry_rs::Point {
                    x: -114.629732,
                    y: 33.032546,
                },
                geometry_rs::Point {
                    x: -114.63419,
                    y: 33.039024,
                },
                geometry_rs::Point {
                    x: -114.639552,
                    y: 33.04529,
                },
                geometry_rs::Point {
                    x: -114.641621,
                    y: 33.046894,
                },
                geometry_rs::Point {
                    x: -114.64482,
                    y: 33.048644,
                },
                geometry_rs::Point {
                    x: -114.645979,
                    y: 33.048902,
                },
                geometry_rs::Point {
                    x: -114.647049,
                    y: 33.048416,
                },
                geometry_rs::Point {
                    x: -114.649001,
                    y: 33.046762,
                },
                geometry_rs::Point {
                    x: -114.650999,
                    y: 33.044131,
                },
                geometry_rs::Point {
                    x: -114.6550380000001,
                    y: 33.037106,
                },
                geometry_rs::Point {
                    x: -114.657827,
                    y: 33.033824,
                },
                geometry_rs::Point {
                    x: -114.659832,
                    y: 33.032664,
                },
                geometry_rs::Point {
                    x: -114.662317,
                    y: 33.03267,
                },
                geometry_rs::Point {
                    x: -114.66506,
                    y: 33.033906,
                },
                geometry_rs::Point {
                    x: -114.670803,
                    y: 33.037983,
                },
                geometry_rs::Point {
                    x: -114.673659,
                    y: 33.041896,
                },
                geometry_rs::Point {
                    x: -114.67483,
                    y: 33.045507,
                },
                geometry_rs::Point {
                    x: -114.6751030000001,
                    y: 33.04753,
                },
                geometry_rs::Point {
                    x: -114.674295,
                    y: 33.057169,
                },
                geometry_rs::Point {
                    x: -114.679114,
                    y: 33.061966,
                },
                geometry_rs::Point {
                    x: -114.686991,
                    y: 33.070968,
                },
                geometry_rs::Point {
                    x: -114.68912,
                    y: 33.076121,
                },
                geometry_rs::Point {
                    x: -114.689307,
                    y: 33.079179,
                },
                geometry_rs::Point {
                    x: -114.6885970000001,
                    y: 33.082869,
                },
                geometry_rs::Point {
                    x: -114.68902,
                    y: 33.084035,
                },
                geometry_rs::Point {
                    x: -114.6925480000001,
                    y: 33.085786,
                },
                geometry_rs::Point {
                    x: -114.694628,
                    y: 33.086226,
                },
                geometry_rs::Point {
                    x: -114.7011650000001,
                    y: 33.086368,
                },
                geometry_rs::Point {
                    x: -114.70473,
                    y: 33.087051,
                },
                geometry_rs::Point {
                    x: -114.706488,
                    y: 33.08816,
                },
                geometry_rs::Point {
                    x: -114.707819,
                    y: 33.091102,
                },
                geometry_rs::Point {
                    x: -114.7081330000001,
                    y: 33.094022,
                },
                geometry_rs::Point {
                    x: -114.7078960000001,
                    y: 33.097431,
                },
                geometry_rs::Point {
                    x: -114.706175,
                    y: 33.105334,
                },
                geometry_rs::Point {
                    x: -114.703682,
                    y: 33.113768,
                },
                geometry_rs::Point {
                    x: -114.696914,
                    y: 33.131119,
                },
                geometry_rs::Point {
                    x: -114.694858,
                    y: 33.13346,
                },
                geometry_rs::Point {
                    x: -114.690246,
                    y: 33.137724,
                },
                geometry_rs::Point {
                    x: -114.687405,
                    y: 33.141983,
                },
                geometry_rs::Point {
                    x: -114.684907,
                    y: 33.147823,
                },
                geometry_rs::Point {
                    x: -114.682759,
                    y: 33.154808,
                },
                geometry_rs::Point {
                    x: -114.679945,
                    y: 33.159059,
                },
                geometry_rs::Point {
                    x: -114.67935,
                    y: 33.162433,
                },
                geometry_rs::Point {
                    x: -114.68089,
                    y: 33.169074,
                },
                geometry_rs::Point {
                    x: -114.680237,
                    y: 33.169637,
                },
                geometry_rs::Point {
                    x: -114.679115,
                    y: 33.174608,
                },
                geometry_rs::Point {
                    x: -114.6758300000001,
                    y: 33.18152,
                },
                geometry_rs::Point {
                    x: -114.6753590000001,
                    y: 33.185488,
                },
                geometry_rs::Point {
                    x: -114.675189,
                    y: 33.188178,
                },
                geometry_rs::Point {
                    x: -114.678163,
                    y: 33.199488,
                },
                geometry_rs::Point {
                    x: -114.678749,
                    y: 33.203448,
                },
                geometry_rs::Point {
                    x: -114.676072,
                    y: 33.210835,
                },
                geometry_rs::Point {
                    x: -114.673715,
                    y: 33.219245,
                },
                geometry_rs::Point {
                    x: -114.673626,
                    y: 33.223121,
                },
                geometry_rs::Point {
                    x: -114.6744790000001,
                    y: 33.225504,
                },
                geometry_rs::Point {
                    x: -114.678097,
                    y: 33.2303,
                },
                geometry_rs::Point {
                    x: -114.682731,
                    y: 33.234918,
                },
                geometry_rs::Point {
                    x: -114.689421,
                    y: 33.24525,
                },
                geometry_rs::Point {
                    x: -114.689541,
                    y: 33.246428,
                },
                geometry_rs::Point {
                    x: -114.688205,
                    y: 33.247965,
                },
                geometry_rs::Point {
                    x: -114.683253,
                    y: 33.250034,
                },
                geometry_rs::Point {
                    x: -114.67766,
                    y: 33.254426,
                },
                geometry_rs::Point {
                    x: -114.6744910000001,
                    y: 33.255597,
                },
                geometry_rs::Point {
                    x: -114.672924,
                    y: 33.257042,
                },
                geometry_rs::Point {
                    x: -114.672088,
                    y: 33.258499,
                },
                geometry_rs::Point {
                    x: -114.672401,
                    y: 33.260469,
                },
                geometry_rs::Point {
                    x: -114.677032,
                    y: 33.270169,
                },
                geometry_rs::Point {
                    x: -114.680507,
                    y: 33.273576,
                },
                geometry_rs::Point {
                    x: -114.6843630000001,
                    y: 33.276023,
                },
                geometry_rs::Point {
                    x: -114.688599,
                    y: 33.277861,
                },
                geometry_rs::Point {
                    x: -114.694449,
                    y: 33.279785,
                },
                geometry_rs::Point {
                    x: -114.702873,
                    y: 33.281916,
                },
                geometry_rs::Point {
                    x: -114.711197,
                    y: 33.283341,
                },
                geometry_rs::Point {
                    x: -114.717875,
                    y: 33.285156,
                },
                geometry_rs::Point {
                    x: -114.72167,
                    y: 33.286982,
                },
                geometry_rs::Point {
                    x: -114.723259,
                    y: 33.288079,
                },
                geometry_rs::Point {
                    x: -114.731223,
                    y: 33.302433,
                },
                geometry_rs::Point {
                    x: -114.7312220000001,
                    y: 33.304039,
                },
                geometry_rs::Point {
                    x: -114.7299040000001,
                    y: 33.305745,
                },
                geometry_rs::Point {
                    x: -114.726484,
                    y: 33.308273,
                },
                geometry_rs::Point {
                    x: -114.7246650000001,
                    y: 33.310097,
                },
                geometry_rs::Point {
                    x: -114.7236230000001,
                    y: 33.312109,
                },
                geometry_rs::Point {
                    x: -114.71861,
                    y: 33.315761,
                },
                geometry_rs::Point {
                    x: -114.710627,
                    y: 33.3205,
                },
                geometry_rs::Point {
                    x: -114.70787,
                    y: 33.323316,
                },
                geometry_rs::Point {
                    x: -114.705186,
                    y: 33.327709,
                },
                geometry_rs::Point {
                    x: -114.700938,
                    y: 33.337014,
                },
                geometry_rs::Point {
                    x: -114.69935,
                    y: 33.345692,
                },
                geometry_rs::Point {
                    x: -114.699124,
                    y: 33.349258,
                },
                geometry_rs::Point {
                    x: -114.698035,
                    y: 33.352442,
                },
                geometry_rs::Point {
                    x: -114.69817,
                    y: 33.356575,
                },
                geometry_rs::Point {
                    x: -114.699056,
                    y: 33.361148,
                },
                geometry_rs::Point {
                    x: -114.701959,
                    y: 33.367134,
                },
                geometry_rs::Point {
                    x: -114.704201,
                    y: 33.371238,
                },
                geometry_rs::Point {
                    x: -114.706722,
                    y: 33.37503,
                },
                geometry_rs::Point {
                    x: -114.707348,
                    y: 33.376627,
                },
                geometry_rs::Point {
                    x: -114.7074850000001,
                    y: 33.378375,
                },
                geometry_rs::Point {
                    x: -114.707009,
                    y: 33.380633,
                },
                geometry_rs::Point {
                    x: -114.707309,
                    y: 33.38254,
                },
                geometry_rs::Point {
                    x: -114.708407,
                    y: 33.384142,
                },
                geometry_rs::Point {
                    x: -114.713602,
                    y: 33.388256,
                },
                geometry_rs::Point {
                    x: -114.72425,
                    y: 33.40042,
                },
                geometry_rs::Point {
                    x: -114.725292,
                    y: 33.402341,
                },
                geometry_rs::Point {
                    x: -114.725535,
                    y: 33.404055,
                },
                geometry_rs::Point {
                    x: -114.725282,
                    y: 33.405048,
                },
                geometry_rs::Point {
                    x: -114.723829,
                    y: 33.406531,
                },
                geometry_rs::Point {
                    x: -114.722201,
                    y: 33.407384,
                },
                geometry_rs::Point {
                    x: -114.7200650000001,
                    y: 33.407891,
                },
                geometry_rs::Point {
                    x: -114.710878,
                    y: 33.407254,
                },
                geometry_rs::Point {
                    x: -114.701788,
                    y: 33.408377,
                },
                geometry_rs::Point {
                    x: -114.697708,
                    y: 33.410942,
                },
                geometry_rs::Point {
                    x: -114.696805,
                    y: 33.412087,
                },
                geometry_rs::Point {
                    x: -114.696507,
                    y: 33.414063,
                },
                geometry_rs::Point {
                    x: -114.695658,
                    y: 33.415128,
                },
                geometry_rs::Point {
                    x: -114.68795,
                    y: 33.417934,
                },
                geometry_rs::Point {
                    x: -114.673691,
                    y: 33.419157,
                },
                geometry_rs::Point {
                    x: -114.658254,
                    y: 33.413021,
                },
                geometry_rs::Point {
                    x: -114.656735,
                    y: 33.412813,
                },
                geometry_rs::Point {
                    x: -114.652828,
                    y: 33.412923,
                },
                geometry_rs::Point {
                    x: -114.64954,
                    y: 33.413633,
                },
                geometry_rs::Point {
                    x: -114.643302,
                    y: 33.416746,
                },
                geometry_rs::Point {
                    x: -114.635183,
                    y: 33.422725,
                },
                geometry_rs::Point {
                    x: -114.633262,
                    y: 33.425024,
                },
                geometry_rs::Point {
                    x: -114.630903,
                    y: 33.426754,
                },
                geometry_rs::Point {
                    x: -114.62964,
                    y: 33.428137,
                },
                geometry_rs::Point {
                    x: -114.627479,
                    y: 33.432307,
                },
                geometry_rs::Point {
                    x: -114.622283,
                    y: 33.447558,
                },
                geometry_rs::Point {
                    x: -114.622519,
                    y: 33.450879,
                },
                geometry_rs::Point {
                    x: -114.6233950000001,
                    y: 33.45449,
                },
                geometry_rs::Point {
                    x: -114.622918,
                    y: 33.456561,
                },
                geometry_rs::Point {
                    x: -114.618354,
                    y: 33.462708,
                },
                geometry_rs::Point {
                    x: -114.614331,
                    y: 33.467315,
                },
                geometry_rs::Point {
                    x: -114.6137820000001,
                    y: 33.469049,
                },
                geometry_rs::Point {
                    x: -114.612472,
                    y: 33.470768,
                },
                geometry_rs::Point {
                    x: -114.6078430000001,
                    y: 33.474834,
                },
                geometry_rs::Point {
                    x: -114.6033960000001,
                    y: 33.480631,
                },
                geometry_rs::Point {
                    x: -114.601694,
                    y: 33.481396,
                },
                geometry_rs::Point {
                    x: -114.599712,
                    y: 33.484316,
                },
                geometry_rs::Point {
                    x: -114.5972830000001,
                    y: 33.490653,
                },
                geometry_rs::Point {
                    x: -114.593721,
                    y: 33.495932,
                },
                geometry_rs::Point {
                    x: -114.592369,
                    y: 33.498675,
                },
                geometry_rs::Point {
                    x: -114.589246,
                    y: 33.501813,
                },
                geometry_rs::Point {
                    x: -114.580468,
                    y: 33.506465,
                },
                geometry_rs::Point {
                    x: -114.573757,
                    y: 33.507543,
                },
                geometry_rs::Point {
                    x: -114.569533,
                    y: 33.509219,
                },
                geometry_rs::Point {
                    x: -114.560963,
                    y: 33.516739,
                },
                geometry_rs::Point {
                    x: -114.560552,
                    y: 33.518272,
                },
                geometry_rs::Point {
                    x: -114.560835,
                    y: 33.524334,
                },
                geometry_rs::Point {
                    x: -114.560098,
                    y: 33.526663,
                },
                geometry_rs::Point {
                    x: -114.559507,
                    y: 33.530724,
                },
                geometry_rs::Point {
                    x: -114.558898,
                    y: 33.531819,
                },
                geometry_rs::Point {
                    x: -114.542011,
                    y: 33.542481,
                },
                geometry_rs::Point {
                    x: -114.531802,
                    y: 33.547862,
                },
                geometry_rs::Point {
                    x: -114.530401,
                    y: 33.550099,
                },
                geometry_rs::Point {
                    x: -114.5259970000001,
                    y: 33.551457,
                },
                geometry_rs::Point {
                    x: -114.524599,
                    y: 33.552231,
                },
                geometry_rs::Point {
                    x: -114.524215,
                    y: 33.553068,
                },
                geometry_rs::Point {
                    x: -114.52822,
                    y: 33.559318,
                },
                geometry_rs::Point {
                    x: -114.531613,
                    y: 33.561702,
                },
                geometry_rs::Point {
                    x: -114.532333,
                    y: 33.562879,
                },
                geometry_rs::Point {
                    x: -114.533192,
                    y: 33.565823,
                },
                geometry_rs::Point {
                    x: -114.5359650000001,
                    y: 33.569154,
                },
                geometry_rs::Point {
                    x: -114.536784,
                    y: 33.570959,
                },
                geometry_rs::Point {
                    x: -114.537801,
                    y: 33.575555,
                },
                geometry_rs::Point {
                    x: -114.538983,
                    y: 33.576792,
                },
                geometry_rs::Point {
                    x: -114.5403,
                    y: 33.580615,
                },
                geometry_rs::Point {
                    x: -114.540652,
                    y: 33.582872,
                },
                geometry_rs::Point {
                    x: -114.540111,
                    y: 33.588354,
                },
                geometry_rs::Point {
                    x: -114.540664,
                    y: 33.589789,
                },
                geometry_rs::Point {
                    x: -114.540617,
                    y: 33.591412,
                },
                geometry_rs::Point {
                    x: -114.537493,
                    y: 33.594895,
                },
                geometry_rs::Point {
                    x: -114.536777,
                    y: 33.596394,
                },
                geometry_rs::Point {
                    x: -114.531051,
                    y: 33.604482,
                },
                geometry_rs::Point {
                    x: -114.529186,
                    y: 33.60665,
                },
                geometry_rs::Point {
                    x: -114.526782,
                    y: 33.608831,
                },
                geometry_rs::Point {
                    x: -114.523994,
                    y: 33.60999,
                },
                geometry_rs::Point {
                    x: -114.522071,
                    y: 33.611277,
                },
                geometry_rs::Point {
                    x: -114.521845,
                    y: 33.612544,
                },
                geometry_rs::Point {
                    x: -114.522367,
                    y: 33.614172,
                },
                geometry_rs::Point {
                    x: -114.527378,
                    y: 33.617828,
                },
                geometry_rs::Point {
                    x: -114.528578,
                    y: 33.619994,
                },
                geometry_rs::Point {
                    x: -114.52908,
                    y: 33.621711,
                },
                geometry_rs::Point {
                    x: -114.531215,
                    y: 33.623913,
                },
                geometry_rs::Point {
                    x: -114.531034,
                    y: 33.628213,
                },
                geometry_rs::Point {
                    x: -114.530311,
                    y: 33.629037,
                },
                geometry_rs::Point {
                    x: -114.52637,
                    y: 33.630259,
                },
                geometry_rs::Point {
                    x: -114.523802,
                    y: 33.6347,
                },
                geometry_rs::Point {
                    x: -114.525394,
                    y: 33.640669,
                },
                geometry_rs::Point {
                    x: -114.5295490000001,
                    y: 33.643861,
                },
                geometry_rs::Point {
                    x: -114.533215,
                    y: 33.648443,
                },
                geometry_rs::Point {
                    x: -114.533194,
                    y: 33.65166,
                },
                geometry_rs::Point {
                    x: -114.532164,
                    y: 33.653194,
                },
                geometry_rs::Point {
                    x: -114.530583,
                    y: 33.654461,
                },
                geometry_rs::Point {
                    x: -114.525163,
                    y: 33.655939,
                },
                geometry_rs::Point {
                    x: -114.518337,
                    y: 33.655927,
                },
                geometry_rs::Point {
                    x: -114.5145590000001,
                    y: 33.658014,
                },
                geometry_rs::Point {
                    x: -114.514057,
                    y: 33.660179,
                },
                geometry_rs::Point {
                    x: -114.515336,
                    y: 33.662033,
                },
                geometry_rs::Point {
                    x: -114.517112,
                    y: 33.662877,
                },
                geometry_rs::Point {
                    x: -114.520671,
                    y: 33.662681,
                },
                geometry_rs::Point {
                    x: -114.526439,
                    y: 33.66388,
                },
                geometry_rs::Point {
                    x: -114.530267,
                    y: 33.666821,
                },
                geometry_rs::Point {
                    x: -114.532123,
                    y: 33.669702,
                },
                geometry_rs::Point {
                    x: -114.531523,
                    y: 33.675108,
                },
                geometry_rs::Point {
                    x: -114.530348,
                    y: 33.679245,
                },
                geometry_rs::Point {
                    x: -114.527782,
                    y: 33.682684,
                },
                geometry_rs::Point {
                    x: -114.523959,
                    y: 33.685879,
                },
                geometry_rs::Point {
                    x: -114.519113,
                    y: 33.688473,
                },
                geometry_rs::Point {
                    x: -114.512409,
                    y: 33.691282,
                },
                geometry_rs::Point {
                    x: -114.507996,
                    y: 33.692018,
                },
                geometry_rs::Point {
                    x: -114.504993,
                    y: 33.693022,
                },
                geometry_rs::Point {
                    x: -114.502899,
                    y: 33.694255,
                },
                geometry_rs::Point {
                    x: -114.496489,
                    y: 33.696901,
                },
                geometry_rs::Point {
                    x: -114.495719,
                    y: 33.698454,
                },
                geometry_rs::Point {
                    x: -114.495537,
                    y: 33.701506,
                },
                geometry_rs::Point {
                    x: -114.494407,
                    y: 33.705395,
                },
                geometry_rs::Point {
                    x: -114.494197,
                    y: 33.707922,
                },
                geometry_rs::Point {
                    x: -114.494901,
                    y: 33.71443,
                },
                geometry_rs::Point {
                    x: -114.496565,
                    y: 33.719155,
                },
                geometry_rs::Point {
                    x: -114.498133,
                    y: 33.720634,
                },
                geometry_rs::Point {
                    x: -114.500788,
                    y: 33.722204,
                },
                geometry_rs::Point {
                    x: -114.502661,
                    y: 33.724584,
                },
                geometry_rs::Point {
                    x: -114.504176,
                    y: 33.728055,
                },
                geometry_rs::Point {
                    x: -114.506799,
                    y: 33.730518,
                },
                geometry_rs::Point {
                    x: -114.510265,
                    y: 33.732146,
                },
                geometry_rs::Point {
                    x: -114.5123480000001,
                    y: 33.734214,
                },
                geometry_rs::Point {
                    x: -114.510777,
                    y: 33.737574,
                },
                geometry_rs::Point {
                    x: -114.508206,
                    y: 33.741587,
                },
                geometry_rs::Point {
                    x: -114.506,
                    y: 33.746344,
                },
                geometry_rs::Point {
                    x: -114.504483,
                    y: 33.750998,
                },
                geometry_rs::Point {
                    x: -114.50434,
                    y: 33.756381,
                },
                geometry_rs::Point {
                    x: -114.504863,
                    y: 33.760465,
                },
                geometry_rs::Point {
                    x: -114.507089,
                    y: 33.76793,
                },
                geometry_rs::Point {
                    x: -114.516734,
                    y: 33.788345,
                },
                geometry_rs::Point {
                    x: -114.518942,
                    y: 33.797302,
                },
                geometry_rs::Point {
                    x: -114.521555,
                    y: 33.801982,
                },
                geometry_rs::Point {
                    x: -114.524682,
                    y: 33.808961,
                },
                geometry_rs::Point {
                    x: -114.527188,
                    y: 33.812639,
                },
                geometry_rs::Point {
                    x: -114.52805,
                    y: 33.814963,
                },
                geometry_rs::Point {
                    x: -114.527886,
                    y: 33.815617,
                },
                geometry_rs::Point {
                    x: -114.527161,
                    y: 33.816191,
                },
                geometry_rs::Point {
                    x: -114.522714,
                    y: 33.818979,
                },
                geometry_rs::Point {
                    x: -114.520733,
                    y: 33.822031,
                },
                geometry_rs::Point {
                    x: -114.51997,
                    y: 33.825381,
                },
                geometry_rs::Point {
                    x: -114.520465,
                    y: 33.827778,
                },
                geometry_rs::Point {
                    x: -114.523409,
                    y: 33.835323,
                },
                geometry_rs::Point {
                    x: -114.525539,
                    y: 33.838614,
                },
                geometry_rs::Point {
                    x: -114.529597,
                    y: 33.848063,
                },
                geometry_rs::Point {
                    x: -114.530607,
                    y: 33.85544,
                },
                geometry_rs::Point {
                    x: -114.529883,
                    y: 33.857563,
                },
                geometry_rs::Point {
                    x: -114.527069,
                    y: 33.859429,
                },
                geometry_rs::Point {
                    x: -114.525666,
                    y: 33.860003,
                },
                geometry_rs::Point {
                    x: -114.524292,
                    y: 33.860133,
                },
                geometry_rs::Point {
                    x: -114.52287,
                    y: 33.859965,
                },
                geometry_rs::Point {
                    x: -114.518998,
                    y: 33.858563,
                },
                geometry_rs::Point {
                    x: -114.5168110000001,
                    y: 33.85812,
                },
                geometry_rs::Point {
                    x: -114.514673,
                    y: 33.858638,
                },
                geometry_rs::Point {
                    x: -114.511346,
                    y: 33.86157,
                },
                geometry_rs::Point {
                    x: -114.506635,
                    y: 33.863484,
                },
                geometry_rs::Point {
                    x: -114.505638,
                    y: 33.864276,
                },
                geometry_rs::Point {
                    x: -114.503887,
                    y: 33.865754,
                },
                geometry_rs::Point {
                    x: -114.503104,
                    y: 33.867166,
                },
                geometry_rs::Point {
                    x: -114.503017,
                    y: 33.867998,
                },
                geometry_rs::Point {
                    x: -114.5038600000001,
                    y: 33.871234,
                },
                geometry_rs::Point {
                    x: -114.503395,
                    y: 33.875018,
                },
                geometry_rs::Point {
                    x: -114.50434,
                    y: 33.876882,
                },
                geometry_rs::Point {
                    x: -114.510138,
                    y: 33.880777,
                },
                geometry_rs::Point {
                    x: -114.51866,
                    y: 33.888263,
                },
                geometry_rs::Point {
                    x: -114.522768,
                    y: 33.892583,
                },
                geometry_rs::Point {
                    x: -114.524813,
                    y: 33.895684,
                },
                geometry_rs::Point {
                    x: -114.525872,
                    y: 33.901008,
                },
                geometry_rs::Point {
                    x: -114.52569,
                    y: 33.901428,
                },
                geometry_rs::Point {
                    x: -114.524289,
                    y: 33.901587,
                },
                geometry_rs::Point {
                    x: -114.516344,
                    y: 33.897918,
                },
                geometry_rs::Point {
                    x: -114.513715,
                    y: 33.897959,
                },
                geometry_rs::Point {
                    x: -114.510944,
                    y: 33.899099,
                },
                geometry_rs::Point {
                    x: -114.508708,
                    y: 33.90064,
                },
                geometry_rs::Point {
                    x: -114.507988,
                    y: 33.901813,
                },
                geometry_rs::Point {
                    x: -114.50792,
                    y: 33.903807,
                },
                geometry_rs::Point {
                    x: -114.508558,
                    y: 33.906098,
                },
                geometry_rs::Point {
                    x: -114.511511,
                    y: 33.911092,
                },
                geometry_rs::Point {
                    x: -114.514503,
                    y: 33.914214,
                },
                geometry_rs::Point {
                    x: -114.518434,
                    y: 33.917518,
                },
                geometry_rs::Point {
                    x: -114.523393,
                    y: 33.921221,
                },
                geometry_rs::Point {
                    x: -114.525361,
                    y: 33.922272,
                },
                geometry_rs::Point {
                    x: -114.528385,
                    y: 33.923674,
                },
                geometry_rs::Point {
                    x: -114.531107,
                    y: 33.924633,
                },
                geometry_rs::Point {
                    x: -114.534146,
                    y: 33.925187,
                },
                geometry_rs::Point {
                    x: -114.534951,
                    y: 33.9257,
                },
                geometry_rs::Point {
                    x: -114.535853,
                    y: 33.928103,
                },
                geometry_rs::Point {
                    x: -114.535478,
                    y: 33.934651,
                },
                geometry_rs::Point {
                    x: -114.530566,
                    y: 33.943629,
                },
                geometry_rs::Point {
                    x: -114.52868,
                    y: 33.947817,
                },
                geometry_rs::Point {
                    x: -114.526353,
                    y: 33.950917,
                },
                geometry_rs::Point {
                    x: -114.522002,
                    y: 33.955623,
                },
                geometry_rs::Point {
                    x: -114.51586,
                    y: 33.958106,
                },
                geometry_rs::Point {
                    x: -114.51497,
                    y: 33.958149,
                },
                geometry_rs::Point {
                    x: -114.511231,
                    y: 33.95704,
                },
                geometry_rs::Point {
                    x: -114.509568,
                    y: 33.957264,
                },
                geometry_rs::Point {
                    x: -114.499883,
                    y: 33.961789,
                },
                geometry_rs::Point {
                    x: -114.496042,
                    y: 33.96589,
                },
                geometry_rs::Point {
                    x: -114.490398,
                    y: 33.97062,
                },
                geometry_rs::Point {
                    x: -114.488459,
                    y: 33.972832,
                },
                geometry_rs::Point {
                    x: -114.484784,
                    y: 33.975519,
                },
                geometry_rs::Point {
                    x: -114.483097,
                    y: 33.977745,
                },
                geometry_rs::Point {
                    x: -114.482333,
                    y: 33.980181,
                },
                geometry_rs::Point {
                    x: -114.481455,
                    y: 33.981261,
                },
                geometry_rs::Point {
                    x: -114.475907,
                    y: 33.984424,
                },
                geometry_rs::Point {
                    x: -114.471138,
                    y: 33.98804,
                },
                geometry_rs::Point {
                    x: -114.467932,
                    y: 33.992877,
                },
                geometry_rs::Point {
                    x: -114.466187,
                    y: 33.993465,
                },
                geometry_rs::Point {
                    x: -114.461133,
                    y: 33.993541,
                },
                geometry_rs::Point {
                    x: -114.46012,
                    y: 33.993888,
                },
                geometry_rs::Point {
                    x: -114.459258,
                    y: 33.994711,
                },
                geometry_rs::Point {
                    x: -114.458028,
                    y: 33.997158,
                },
                geometry_rs::Point {
                    x: -114.4580260000001,
                    y: 33.99782,
                },
                geometry_rs::Point {
                    x: -114.459184,
                    y: 34.000016,
                },
                geometry_rs::Point {
                    x: -114.460689,
                    y: 34.001128,
                },
                geometry_rs::Point {
                    x: -114.46628,
                    y: 34.003885,
                },
                geometry_rs::Point {
                    x: -114.46731,
                    y: 34.00519,
                },
                geometry_rs::Point {
                    x: -114.467404,
                    y: 34.00745,
                },
                geometry_rs::Point {
                    x: -114.4658670000001,
                    y: 34.010987,
                },
                geometry_rs::Point {
                    x: -114.464525,
                    y: 34.011982,
                },
                geometry_rs::Point {
                    x: -114.463336,
                    y: 34.012259,
                },
                geometry_rs::Point {
                    x: -114.454807,
                    y: 34.010968,
                },
                geometry_rs::Point {
                    x: -114.450206,
                    y: 34.012574,
                },
                geometry_rs::Point {
                    x: -114.446815,
                    y: 34.01421,
                },
                geometry_rs::Point {
                    x: -114.443821,
                    y: 34.016176,
                },
                geometry_rs::Point {
                    x: -114.44054,
                    y: 34.019329,
                },
                geometry_rs::Point {
                    x: -114.438266,
                    y: 34.022609,
                },
                geometry_rs::Point {
                    x: -114.436171,
                    y: 34.028083,
                },
                geometry_rs::Point {
                    x: -114.434949,
                    y: 34.037784,
                },
                geometry_rs::Point {
                    x: -114.435816,
                    y: 34.04373,
                },
                geometry_rs::Point {
                    x: -114.438602,
                    y: 34.050205,
                },
                geometry_rs::Point {
                    x: -114.439406,
                    y: 34.05381,
                },
                geometry_rs::Point {
                    x: -114.43934,
                    y: 34.057893,
                },
                geometry_rs::Point {
                    x: -114.437683,
                    y: 34.071937,
                },
                geometry_rs::Point {
                    x: -114.435907,
                    y: 34.077491,
                },
                geometry_rs::Point {
                    x: -114.434181,
                    y: 34.087379,
                },
                geometry_rs::Point {
                    x: -114.43338,
                    y: 34.088413,
                },
                geometry_rs::Point {
                    x: -114.428026,
                    y: 34.092787,
                },
                geometry_rs::Point {
                    x: -114.426168,
                    y: 34.097042,
                },
                geometry_rs::Point {
                    x: -114.422899,
                    y: 34.099661,
                },
                geometry_rs::Point {
                    x: -114.420499,
                    y: 34.103466,
                },
                geometry_rs::Point {
                    x: -114.415908,
                    y: 34.107636,
                },
                geometry_rs::Point {
                    x: -114.41168,
                    y: 34.110031,
                },
                geometry_rs::Point {
                    x: -114.405916,
                    y: 34.111468,
                },
                geometry_rs::Point {
                    x: -114.401336,
                    y: 34.111638,
                },
                geometry_rs::Point {
                    x: -114.390565,
                    y: 34.110084,
                },
                geometry_rs::Point {
                    x: -114.379223,
                    y: 34.11599,
                },
                geometry_rs::Point {
                    x: -114.3692920000001,
                    y: 34.117519,
                },
                geometry_rs::Point {
                    x: -114.366517,
                    y: 34.118577,
                },
                geometry_rs::Point {
                    x: -114.360402,
                    y: 34.123577,
                },
                geometry_rs::Point {
                    x: -114.359389,
                    y: 34.125016,
                },
                geometry_rs::Point {
                    x: -114.358358,
                    y: 34.127617,
                },
                geometry_rs::Point {
                    x: -114.356372,
                    y: 34.130428,
                },
                geometry_rs::Point {
                    x: -114.35303,
                    y: 34.13312,
                },
                geometry_rs::Point {
                    x: -114.350478,
                    y: 34.134107,
                },
                geometry_rs::Point {
                    x: -114.348051,
                    y: 34.134457,
                },
                geometry_rs::Point {
                    x: -114.336112,
                    y: 34.134034,
                },
                geometry_rs::Point {
                    x: -114.324576,
                    y: 34.136759,
                },
                geometry_rs::Point {
                    x: -114.320777,
                    y: 34.138635,
                },
                geometry_rs::Point {
                    x: -114.312206,
                    y: 34.144776,
                },
                geometry_rs::Point {
                    x: -114.307802,
                    y: 34.150574,
                },
                geometry_rs::Point {
                    x: -114.298168,
                    y: 34.160321,
                },
                geometry_rs::Point {
                    x: -114.292806,
                    y: 34.166725,
                },
                geometry_rs::Point {
                    x: -114.287294,
                    y: 34.170529,
                },
                geometry_rs::Point {
                    x: -114.275267,
                    y: 34.172149,
                },
                geometry_rs::Point {
                    x: -114.26846,
                    y: 34.170177,
                },
                geometry_rs::Point {
                    x: -114.257034,
                    y: 34.172837,
                },
                geometry_rs::Point {
                    x: -114.2534440000001,
                    y: 34.174129,
                },
                geometry_rs::Point {
                    x: -114.244421,
                    y: 34.179403,
                },
                geometry_rs::Point {
                    x: -114.2407120000001,
                    y: 34.183232,
                },
                geometry_rs::Point {
                    x: -114.229715,
                    y: 34.186928,
                },
                geometry_rs::Point {
                    x: -114.227034,
                    y: 34.188866,
                },
                geometry_rs::Point {
                    x: -114.225814,
                    y: 34.191238,
                },
                geometry_rs::Point {
                    x: -114.224941,
                    y: 34.193896,
                },
                geometry_rs::Point {
                    x: -114.225075,
                    y: 34.196814,
                },
                geometry_rs::Point {
                    x: -114.22579,
                    y: 34.199236,
                },
                geometry_rs::Point {
                    x: -114.225861,
                    y: 34.201774,
                },
                geometry_rs::Point {
                    x: -114.225194,
                    y: 34.203642,
                },
                geometry_rs::Point {
                    x: -114.223384,
                    y: 34.205136,
                },
                geometry_rs::Point {
                    x: -114.215454,
                    y: 34.208956,
                },
                geometry_rs::Point {
                    x: -114.211761,
                    y: 34.211539,
                },
                geometry_rs::Point {
                    x: -114.208253,
                    y: 34.215505,
                },
                geometry_rs::Point {
                    x: -114.190876,
                    y: 34.230858,
                },
                geometry_rs::Point {
                    x: -114.17805,
                    y: 34.239969,
                },
                geometry_rs::Point {
                    x: -114.176403,
                    y: 34.241512,
                },
                geometry_rs::Point {
                    x: -114.175948,
                    y: 34.242695,
                },
                geometry_rs::Point {
                    x: -114.175906,
                    y: 34.245587,
                },
                geometry_rs::Point {
                    x: -114.174597,
                    y: 34.247303,
                },
                geometry_rs::Point {
                    x: -114.166536,
                    y: 34.249647,
                },
                geometry_rs::Point {
                    x: -114.166124,
                    y: 34.250015,
                },
                geometry_rs::Point {
                    x: -114.164476,
                    y: 34.251667,
                },
                geometry_rs::Point {
                    x: -114.163867,
                    y: 34.253349,
                },
                geometry_rs::Point {
                    x: -114.163959,
                    y: 34.255377,
                },
                geometry_rs::Point {
                    x: -114.165335,
                    y: 34.258486,
                },
                geometry_rs::Point {
                    x: -114.165249,
                    y: 34.259125,
                },
                geometry_rs::Point {
                    x: -114.164648,
                    y: 34.259699,
                },
                geometry_rs::Point {
                    x: -114.156853,
                    y: 34.258415,
                },
                geometry_rs::Point {
                    x: -114.153346,
                    y: 34.258289,
                },
                geometry_rs::Point {
                    x: -114.147159,
                    y: 34.259564,
                },
                geometry_rs::Point {
                    x: -114.144779,
                    y: 34.259623,
                },
                geometry_rs::Point {
                    x: -114.13545,
                    y: 34.257886,
                },
                geometry_rs::Point {
                    x: -114.133264,
                    y: 34.258462,
                },
                geometry_rs::Point {
                    x: -114.1314890000001,
                    y: 34.260387,
                },
                geometry_rs::Point {
                    x: -114.131211,
                    y: 34.26273,
                },
                geometry_rs::Point {
                    x: -114.134768,
                    y: 34.268965,
                },
                geometry_rs::Point {
                    x: -114.136671,
                    y: 34.274377,
                },
                geometry_rs::Point {
                    x: -114.137045,
                    y: 34.277018,
                },
                geometry_rs::Point {
                    x: -114.13605,
                    y: 34.280833,
                },
                geometry_rs::Point {
                    x: -114.136677,
                    y: 34.283936,
                },
                geometry_rs::Point {
                    x: -114.138365,
                    y: 34.288564,
                },
                geometry_rs::Point {
                    x: -114.139534,
                    y: 34.295844,
                },
                geometry_rs::Point {
                    x: -114.139187,
                    y: 34.298074,
                },
                geometry_rs::Point {
                    x: -114.138167,
                    y: 34.300936,
                },
                geometry_rs::Point {
                    x: -114.138282,
                    y: 34.30323,
                },
                geometry_rs::Point {
                    x: -114.14093,
                    y: 34.305919,
                },
                geometry_rs::Point {
                    x: -114.157206,
                    y: 34.317862,
                },
                geometry_rs::Point {
                    x: -114.157939,
                    y: 34.320277,
                },
                geometry_rs::Point {
                    x: -114.164249,
                    y: 34.330816,
                },
                geometry_rs::Point {
                    x: -114.168807,
                    y: 34.339513,
                },
                geometry_rs::Point {
                    x: -114.172845,
                    y: 34.344979,
                },
                geometry_rs::Point {
                    x: -114.176909,
                    y: 34.349306,
                },
                geometry_rs::Point {
                    x: -114.181145,
                    y: 34.352186,
                },
                geometry_rs::Point {
                    x: -114.185556,
                    y: 34.354386,
                },
                geometry_rs::Point {
                    x: -114.191094,
                    y: 34.356125,
                },
                geometry_rs::Point {
                    x: -114.19648,
                    y: 34.359187,
                },
                geometry_rs::Point {
                    x: -114.199482,
                    y: 34.361373,
                },
                geometry_rs::Point {
                    x: -114.213774,
                    y: 34.36246,
                },
                geometry_rs::Point {
                    x: -114.226107,
                    y: 34.365916,
                },
                geometry_rs::Point {
                    x: -114.229686,
                    y: 34.368908,
                },
                geometry_rs::Point {
                    x: -114.233065,
                    y: 34.375013,
                },
                geometry_rs::Point {
                    x: -114.234275,
                    y: 34.376662,
                },
                geometry_rs::Point {
                    x: -114.245261,
                    y: 34.385659,
                },
                geometry_rs::Point {
                    x: -114.248649,
                    y: 34.388113,
                },
                geometry_rs::Point {
                    x: -114.252739,
                    y: 34.3901,
                },
                geometry_rs::Point {
                    x: -114.25822,
                    y: 34.395046,
                },
                geometry_rs::Point {
                    x: -114.262909,
                    y: 34.400373,
                },
                geometry_rs::Point {
                    x: -114.2643170000001,
                    y: 34.401329,
                },
                geometry_rs::Point {
                    x: -114.267521,
                    y: 34.402486,
                },
                geometry_rs::Point {
                    x: -114.272184,
                    y: 34.402961,
                },
                geometry_rs::Point {
                    x: -114.280108,
                    y: 34.403147,
                },
                geometry_rs::Point {
                    x: -114.282261,
                    y: 34.403641,
                },
                geometry_rs::Point {
                    x: -114.286802,
                    y: 34.40534,
                },
                geometry_rs::Point {
                    x: -114.288663,
                    y: 34.406623,
                },
                geometry_rs::Point {
                    x: -114.290219,
                    y: 34.408291,
                },
                geometry_rs::Point {
                    x: -114.291751,
                    y: 34.411104,
                },
                geometry_rs::Point {
                    x: -114.291903,
                    y: 34.416231,
                },
                geometry_rs::Point {
                    x: -114.292226,
                    y: 34.417606,
                },
                geometry_rs::Point {
                    x: -114.294836,
                    y: 34.421389,
                },
                geometry_rs::Point {
                    x: -114.3010160000001,
                    y: 34.426807,
                },
                geometry_rs::Point {
                    x: -114.308659,
                    y: 34.430485,
                },
                geometry_rs::Point {
                    x: -114.312251,
                    y: 34.432726,
                },
                geometry_rs::Point {
                    x: -114.319054,
                    y: 34.435831,
                },
                geometry_rs::Point {
                    x: -114.32613,
                    y: 34.437251,
                },
                geometry_rs::Point {
                    x: -114.32688,
                    y: 34.438048,
                },
                geometry_rs::Point {
                    x: -114.330669,
                    y: 34.445295,
                },
                geometry_rs::Point {
                    x: -114.332991,
                    y: 34.448082,
                },
                geometry_rs::Point {
                    x: -114.335372,
                    y: 34.450038,
                },
                geometry_rs::Point {
                    x: -114.339627,
                    y: 34.451435,
                },
                geometry_rs::Point {
                    x: -114.342615,
                    y: 34.451442,
                },
                geometry_rs::Point {
                    x: -114.348974,
                    y: 34.450166,
                },
                geometry_rs::Point {
                    x: -114.356025,
                    y: 34.449744,
                },
                geometry_rs::Point {
                    x: -114.3634040000001,
                    y: 34.447773,
                },
                geometry_rs::Point {
                    x: -114.373719,
                    y: 34.446938,
                },
                geometry_rs::Point {
                    x: -114.375789,
                    y: 34.447798,
                },
                geometry_rs::Point {
                    x: -114.378852,
                    y: 34.450376,
                },
                geometry_rs::Point {
                    x: -114.382985,
                    y: 34.453971,
                },
                geometry_rs::Point {
                    x: -114.386699,
                    y: 34.457911,
                },
                geometry_rs::Point {
                    x: -114.387407,
                    y: 34.460492,
                },
                geometry_rs::Point {
                    x: -114.387187,
                    y: 34.462021,
                },
                geometry_rs::Point {
                    x: -114.383525,
                    y: 34.470405,
                },
                geometry_rs::Point {
                    x: -114.381701,
                    y: 34.47604,
                },
                geometry_rs::Point {
                    x: -114.381555,
                    y: 34.477883,
                },
                geometry_rs::Point {
                    x: -114.383038,
                    y: 34.488903,
                },
                geometry_rs::Point {
                    x: -114.382358,
                    y: 34.495758,
                },
                geometry_rs::Point {
                    x: -114.381402,
                    y: 34.499245,
                },
                geometry_rs::Point {
                    x: -114.378124,
                    y: 34.507288,
                },
                geometry_rs::Point {
                    x: -114.378223,
                    y: 34.516521,
                },
                geometry_rs::Point {
                    x: -114.380838,
                    y: 34.529724,
                },
                geometry_rs::Point {
                    x: -114.389603,
                    y: 34.542982,
                },
                geometry_rs::Point {
                    x: -114.398847,
                    y: 34.559149,
                },
                geometry_rs::Point {
                    x: -114.405228,
                    y: 34.569637,
                },
                geometry_rs::Point {
                    x: -114.422382,
                    y: 34.580711,
                },
                geometry_rs::Point {
                    x: -114.435671,
                    y: 34.593841,
                },
                geometry_rs::Point {
                    x: -114.43681,
                    y: 34.596074,
                },
                geometry_rs::Point {
                    x: -114.436363,
                    y: 34.596797,
                },
                geometry_rs::Point {
                    x: -114.427502,
                    y: 34.599227,
                },
                geometry_rs::Point {
                    x: -114.425338,
                    y: 34.600842,
                },
                geometry_rs::Point {
                    x: -114.424326,
                    y: 34.602338,
                },
                geometry_rs::Point {
                    x: -114.424202,
                    y: 34.610453,
                },
                geometry_rs::Point {
                    x: -114.428648,
                    y: 34.614641,
                },
                geometry_rs::Point {
                    x: -114.438739,
                    y: 34.621455,
                },
                geometry_rs::Point {
                    x: -114.439495,
                    y: 34.625858,
                },
                geometry_rs::Point {
                    x: -114.441398,
                    y: 34.630171,
                },
                geometry_rs::Point {
                    x: -114.441525,
                    y: 34.631529,
                },
                geometry_rs::Point {
                    x: -114.440685,
                    y: 34.634739,
                },
                geometry_rs::Point {
                    x: -114.440294,
                    y: 34.63824,
                },
                geometry_rs::Point {
                    x: -114.440519,
                    y: 34.640066,
                },
                geometry_rs::Point {
                    x: -114.441465,
                    y: 34.64253,
                },
                geometry_rs::Point {
                    x: -114.444276,
                    y: 34.646542,
                },
                geometry_rs::Point {
                    x: -114.445664,
                    y: 34.647542,
                },
                geometry_rs::Point {
                    x: -114.449549,
                    y: 34.651423,
                },
                geometry_rs::Point {
                    x: -114.457985,
                    y: 34.657113,
                },
                geometry_rs::Point {
                    x: -114.45821,
                    y: 34.657994,
                },
                geometry_rs::Point {
                    x: -114.457702,
                    y: 34.659328,
                },
                geometry_rs::Point {
                    x: -114.457185,
                    y: 34.659992,
                },
                geometry_rs::Point {
                    x: -114.451785,
                    y: 34.663891,
                },
                geometry_rs::Point {
                    x: -114.450614,
                    y: 34.665793,
                },
                geometry_rs::Point {
                    x: -114.450506,
                    y: 34.666836,
                },
                geometry_rs::Point {
                    x: -114.451532,
                    y: 34.668605,
                },
                geometry_rs::Point {
                    x: -114.454305,
                    y: 34.671234,
                },
                geometry_rs::Point {
                    x: -114.45491,
                    y: 34.673092,
                },
                geometry_rs::Point {
                    x: -114.454881,
                    y: 34.675639,
                },
                geometry_rs::Point {
                    x: -114.455536,
                    y: 34.677335,
                },
                geometry_rs::Point {
                    x: -114.458163,
                    y: 34.681161,
                },
                geometry_rs::Point {
                    x: -114.462178,
                    y: 34.6858,
                },
                geometry_rs::Point {
                    x: -114.463633,
                    y: 34.68794,
                },
                geometry_rs::Point {
                    x: -114.465246,
                    y: 34.691202,
                },
                geometry_rs::Point {
                    x: -114.46809,
                    y: 34.701786,
                },
                geometry_rs::Point {
                    x: -114.46813,
                    y: 34.704445,
                },
                geometry_rs::Point {
                    x: -114.46862,
                    y: 34.707573,
                },
                geometry_rs::Point {
                    x: -114.470477,
                    y: 34.711368,
                },
                geometry_rs::Point {
                    x: -114.47162,
                    y: 34.712966,
                },
                geometry_rs::Point {
                    x: -114.473682,
                    y: 34.713964,
                },
                geometry_rs::Point {
                    x: -114.477297,
                    y: 34.714514,
                },
                geometry_rs::Point {
                    x: -114.482779,
                    y: 34.714511,
                },
                geometry_rs::Point {
                    x: -114.487508,
                    y: 34.716626,
                },
                geometry_rs::Point {
                    x: -114.489287,
                    y: 34.720155,
                },
                geometry_rs::Point {
                    x: -114.490971,
                    y: 34.724848,
                },
                geometry_rs::Point {
                    x: -114.492017,
                    y: 34.725702,
                },
                geometry_rs::Point {
                    x: -114.495858,
                    y: 34.727956,
                },
                geometry_rs::Point {
                    x: -114.499007,
                    y: 34.729096,
                },
                geometry_rs::Point {
                    x: -114.500795,
                    y: 34.730418,
                },
                geometry_rs::Point {
                    x: -114.510292,
                    y: 34.733582,
                },
                geometry_rs::Point {
                    x: -114.514178,
                    y: 34.735288,
                },
                geometry_rs::Point {
                    x: -114.516619,
                    y: 34.736745,
                },
                geometry_rs::Point {
                    x: -114.521048,
                    y: 34.741173,
                },
                geometry_rs::Point {
                    x: -114.522619,
                    y: 34.74373,
                },
                geometry_rs::Point {
                    x: -114.525611,
                    y: 34.747005,
                },
                geometry_rs::Point {
                    x: -114.529079,
                    y: 34.750006,
                },
                geometry_rs::Point {
                    x: -114.529615,
                    y: 34.750822,
                },
                geometry_rs::Point {
                    x: -114.540306,
                    y: 34.757109,
                },
                geometry_rs::Point {
                    x: -114.546884,
                    y: 34.761802,
                },
                geometry_rs::Point {
                    x: -114.552682,
                    y: 34.766871,
                },
                geometry_rs::Point {
                    x: -114.558653,
                    y: 34.773852,
                },
                geometry_rs::Point {
                    x: -114.563979,
                    y: 34.782597,
                },
                geometry_rs::Point {
                    x: -114.565184,
                    y: 34.785976,
                },
                geometry_rs::Point {
                    x: -114.569383,
                    y: 34.791568,
                },
                geometry_rs::Point {
                    x: -114.5710100000001,
                    y: 34.794294,
                },
                geometry_rs::Point {
                    x: -114.574474,
                    y: 34.804214,
                },
                geometry_rs::Point {
                    x: -114.574694,
                    y: 34.807471,
                },
                geometry_rs::Point {
                    x: -114.576452,
                    y: 34.8153,
                },
                geometry_rs::Point {
                    x: -114.5786810000001,
                    y: 34.820977,
                },
                geometry_rs::Point {
                    x: -114.581126,
                    y: 34.826115,
                },
                geometry_rs::Point {
                    x: -114.586842,
                    y: 34.835672,
                },
                geometry_rs::Point {
                    x: -114.592339,
                    y: 34.841153,
                },
                geometry_rs::Point {
                    x: -114.600653,
                    y: 34.847361,
                },
                geometry_rs::Point {
                    x: -114.604255,
                    y: 34.849573,
                },
                geometry_rs::Point {
                    x: -114.619878,
                    y: 34.856873,
                },
                geometry_rs::Point {
                    x: -114.623939,
                    y: 34.859738,
                },
                geometry_rs::Point {
                    x: -114.628276,
                    y: 34.863596,
                },
                geometry_rs::Point {
                    x: -114.6306820000001,
                    y: 34.866352,
                },
                geometry_rs::Point {
                    x: -114.633051,
                    y: 34.869971,
                },
                geometry_rs::Point {
                    x: -114.634382,
                    y: 34.87289,
                },
                geometry_rs::Point {
                    x: -114.635176,
                    y: 34.875003,
                },
                geometry_rs::Point {
                    x: -114.635458,
                    y: 34.876902,
                },
            ],
            vec![],
            true,
        );
        return poly;
    }

    #[bench]
    fn poly_contain_point(b: &mut Bencher) {
        let poly = load_poly();
        let p_in = geometry_rs::Point { x: -112.0, y: 33.0 };

        b.iter(|| {
            let _ = poly.contains_point(p_in);
        });
    }

    #[bench]
    fn poly_not_contain_point(b: &mut Bencher) {
        let poly = load_poly();
        let p_in = geometry_rs::Point {
            x: -114.477539062,
            y: 33.99802726,
        };

        b.iter(|| {
            let _ = poly.contains_point(p_in);
        });
    }
}
