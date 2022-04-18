use log::info;
use std::fs::File;
use std::io::{BufReader, Read};
use rusty_docs::gtsx::{self, GenerateTsx};
use rusty_docs::types::documentation::Doc;

pub fn init(mut doc_file: String) -> Vec<Doc> {
    let file = File::open(doc_file).expect("Could not read file.");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    let _bytes: usize = reader
        .read_to_string(&mut contents)
        .expect("The File was corrupted.");
    let res: Vec<Doc> = serde_json::from_str(&contents).expect("Unable to parse");
    info!("{:#?}", res);
    return res;
}

pub fn generate_tsx_file(filename: String, full_docs: Vec<Doc>) -> String {
    let mut complete: String = "".to_string();
    for doc in full_docs {
        match doc {
            Doc::NoneType(n) => {
                complete += &(" ".to_string()
                    + &gtsx::tag_wrap(
                        "p".to_string(),
                        "".to_string(),
                        format!(
                            "{} {}",
                            &gtsx::tag_wrap("b".to_string(), "".to_string(), n.name),
                            n.value.unwrap_or(" ".to_string())
                        ),
                    ))
                    .to_string();
            }
            Doc::FuncType(f) => complete += &(f.generate()),
            Doc::ErrorType(e) => complete += &(e.generate())
        }
    }
    complete = gtsx::title(format!("{}", filename), "".to_string()) + &" ".to_string() + &complete;
    return gtsx::wrapper(filename, complete);
}
