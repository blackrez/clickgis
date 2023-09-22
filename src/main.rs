use clap::{arg, command};
use clickgis::handle;
use std::io;

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
    handle(function.as_str(), io::stdin().lock(), io::stdout());
}
