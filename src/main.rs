use clap::{Arg, Command};
use log::info;
use std::fs::File;
use std::io::prelude::*;
use rusty_docs::types;

mod read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let matches = Command::new("RustyDocs")
        .version("0.1.0")
        .author("Adam Kalinowski <adamkali@outlook.com>")
        .about("Teaches argument parsing")
        .arg(Arg::new("doc")
            .short('d')
            .long("doc")
            .takes_value(true)
            .help("Gets the file to use as the doc parser. If none is provided, the parser will assume the documentation name is documentation.json"))
        .arg(Arg::new("generate")
            .short('g')
            .long("gen")
            .takes_value(false)
            .help("Generates a file based on the documentation provided. Only tsx is supported."))
        .arg(Arg::new("output")
            .short('o')
            .long("out")
            .takes_value(true)
            .help("The output of the file, can be specified by the user to have a specific name or will default to be documentation.tsx for example."))
        .get_matches();

    let doc_file = matches.value_of("doc").unwrap_or("documentation.json");
    let output_filename = matches.value_of("output").unwrap_or("documentation.json");
    info!("Finished Parsing.");
    let mut file = File::create(output_filename.to_string())?;
    let output_name = &output_filename.replace(".tsx", "");

    let docs: Vec<types::documentation::Doc> = read::init(doc_file.to_string());
    info!("{:?}", &docs);
    let output_tsx: String = read::generate_tsx_file(output_name.to_string(), docs);
    let _u = file.write(output_tsx.as_bytes())?;
    return Ok(());
}
