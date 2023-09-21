use geozero::wkb::{Wkb};
use geozero::wkt::{WktReader, WktWriter};
use clap::{arg, command};
use geo::{Geometry};
use geozero::{ToWkt, ToGeo, ToJson};
use std::convert::TryFrom;
use std::io::{self, BufRead};

fn st_aswkt(hexwkb: &str) -> String{
    let wkb = Wkb(hex::decode(&hexwkb).unwrap());
    return wkb.to_wkt().unwrap()
    
}

fn st_asgeojson(hexwkb: &str) -> String {
    let wkb = Wkb(hex::decode(&hexwkb).unwrap());
    return wkb.to_json().unwrap()
}

/**Useful function for debug
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
**/

fn st_geometrytype(hexwkb: &str) -> String {
    // Actually it takes a wkb because their is not geo type defined.
    // I will change when #2 will resolved
    let geom = Wkb(hex::decode(&hexwkb).unwrap()).to_geo().unwrap();
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
        Geometry::Triangle(_) => "Triangle"
    };
    return type_geom.to_string();
}
/**
fn st_contains(hexwkb1: String, hexwkb2: String) -> bool {
    let geo1 = Wkb(hex::decode(&hexwkb1).unwrap());
    let geo2 = Wkb(hex::decode(&hexwkb2).unwrap());
    let contains  =  geo1.to_geo().contains(geo2.to_geo());

    return contains
}
**/
fn main() {
    let matches = command!()
        //.arg(arg!(--geom <VALUE>).required(false))
        .arg(arg!(--function <VALUE>).required(true))
        .get_matches();
    //geom was used as a paremeter in command line
    //let geom = matches.get_one::<String>("geom").expect("required");
    let function = matches.get_one::<String>("function").expect("required");
    //println!("function: {:?}", function);
    //println!("geom: {:?}", geom);
    //let geom = "0101000000E789E76C011115407A6EA12B11D34540";
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n == 0 {
                    // End files
                    break;
                }
                // Traitement de l'entrée
                let result = match function.as_str() {
                    "st_asgeojson" => st_asgeojson(input.trim_end_matches('\n')),
                    "st_aswkt" => st_aswkt(input.trim_end_matches('\n')),
                    "st_geometrytype" => st_geometrytype(input.trim_end_matches('\n')),
                    _ => panic!("not implemented"),
                };
                println!("{}", result);
            }
            Err(error) => {
                println!("Read error : {}", error);
                break;
            }
        }
    }
}
