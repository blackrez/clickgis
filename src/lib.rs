use geo::Geometry;
use geozero::wkb::Wkb;
use geozero::{ToGeo, ToJson, ToWkt};
use std::io::{BufRead, Write};

#[inline]
fn wkb(hexwkb: &str) -> Wkb {
    Wkb(hex::decode(&hexwkb).unwrap())
}

fn st_aswkt(hexwkb: &str) -> String {
    wkb(&hexwkb).to_wkt().unwrap()
}

fn st_asgeojson(hexwkb: &str) -> String {
    wkb(&hexwkb).to_json().unwrap()
}

fn st_geometrytype(hexwkb: &str) -> String {
    // Actually it takes a wkb because their is not geo type defined.
    // I will change when #2 will resolved
    let geom = wkb(&hexwkb).to_geo().unwrap();
    let type_geom = match geom {
        Geometry::Point(_) => "Point",
        Geometry::Line(_) => "Line",
        Geometry::LineString(_) => "LineString",
        Geometry::Polygon(_) => "Polygon",
        Geometry::MultiPoint(_) => "MultiPoint",
        Geometry::MultiLineString(_) => "MultiLineString",
        Geometry::MultiPolygon(_) => "MultiPolygon",
        Geometry::GeometryCollection(_) => "GeometryCollection",
        Geometry::Rect(_) => "Rect",
        Geometry::Triangle(_) => "Triangle",
    };
    return type_geom.to_string();
}

pub fn handle<R, W>(function: &str, mut reader: R, mut writer: W)
where
    R: BufRead,
    W: Write,
{
    let convert = match function {
        "st_asgeojson" => st_asgeojson,
        "st_aswkt" => st_aswkt,
        "st_geometrytype" => st_geometrytype,
        _ => panic!("not implemented"),
    };

    let mut input = String::new();
    loop {
        match reader.read_line(&mut input) {
            Ok(0) => {
                break;
            } // File end
            Ok(_) => {
                // Traitement de l'entrée
                let result = convert(input.trim_end_matches('\n'));
                writeln!(writer, "{}", result).expect("Failed to write to writer");
                input.clear();
            }
            Err(error) => {
                writeln!(writer, "Read error : {}", error).expect("Failed to write to writer");
                break;
            }
        }
    }
}

#[test]
fn test_st_asgeojson() {
    let input = b"0101000000cade52ce17d32740bd35b05582814b40";
    let mut output = Vec::new();

    handle("st_asgeojson", &input[..], &mut output);

    let output = String::from_utf8(output).expect("Not UTF-8");
    assert_eq!(
        "{\"type\": \"Point\", \"coordinates\": [11.912291,55.01179]}\n",
        output
    );
}
#[test]
fn test_st_aswkt() {
    let input = b"0101000000cade52ce17d32740bd35b05582814b40";
    let mut output = Vec::new();

    handle("st_aswkt", &input[..], &mut output);

    let output = String::from_utf8(output).expect("Not UTF-8");
    assert_eq!("POINT(11.912291 55.01179)\n", output);
}
#[test]
fn test_st_geometrytype() {
    let input = b"0101000000cade52ce17d32740bd35b05582814b40";
    let mut output = Vec::new();

    handle("st_geometrytype", &input[..], &mut output);

    let output = String::from_utf8(output).expect("Not UTF-8");
    assert_eq!("Point\n", output);
}
#[test]
fn test_multiple_st_asgeojson() {
    let input = b"0101000000cade52ce17d32740bd35b05582814b40\n".repeat(5);
    let mut output = Vec::new();

    handle("st_asgeojson", &input[..], &mut output);

    let output = String::from_utf8(output).expect("Not UTF-8");
    assert_eq!(
        "{\"type\": \"Point\", \"coordinates\": [11.912291,55.01179]}\n".repeat(5),
        output
    );
}
