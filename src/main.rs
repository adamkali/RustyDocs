use rustc_serialize::json::Json;

use clap::{Arg, App};
use std::env;

mod read;
mod types;

fn main() {
    let mut documentation_json: Json = Json::Null;

    let matches = App::new("My Test Program")
    .version("0.1.0")
    .author("Adam Kalinowski <adamkali@outlook.com>")
    .about("Teaches argument parsing")
    .arg(Arg::new("doc")
             .short('d')
             .long("doc")
             .takes_value(false)
             .help("Gets the file to use as the doc parser."))

    .get_matches();

    let doc_file = matches.value_of("doc").unwrap_or("document.json");

    Documentation::init()

}
