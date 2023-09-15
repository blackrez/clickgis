use geozero::wkb::{Wkb};
use geozero::wkt::{WktReader, WktWriter};
use clap::{arg, command};
use geozero::{ToWkt, ToGeo, ToJson};
use std::convert::TryFrom;
use std::io::{self, BufRead};

fn st_fromwkb(hexwkb: &str) -> String{
    let wkb = Wkb(hex::decode(hexwkb.to_string()).unwrap());
    return wkb.to_wkt().unwrap()
    
}

fn st_asgeojson(hexwkb: &str) -> String {
    let wkb = Wkb(hex::decode(hexwkb.to_string()).unwrap());
    return wkb.to_json().unwrap()
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
                let mut result = match function.as_str() {
                    "st_asgeojson" => st_asgeojson(input.trim_end_matches('\n')),
                    "st_fromwkb" => st_fromwkb(input.trim_end_matches('\n')),
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
