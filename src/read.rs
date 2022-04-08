use crate::Json::Null;
use crate::types::{error_type::ErrorType, doc_type::FuncType};

// Built-ins
extern crate rustc_serialize;
use rustc_serialize::json::Json;
use std::fs::File;
use std::io::Read;

struct Documentation {
    documentation_json: Json,
    file: String,
    function_type: FuncType,
    error_type: ErrorType,
}

impl Documentation {
    pub fn new() -> Documentation {
        Documentation {
            documentation_json: Null,
            file: "./documentation.json".to_string(),
            function_type: FuncType::new(),
            error_type: ErrorType::new(),
        }
    }

    pub fn init(doc_file: String ) -> Result<(), Box<dyn std::error::Error>> {
        let mut doc = Documentation::new();

        doc.file = doc_file;

        let mut f = File::open(&doc.file);

        let json = match f {
            Ok(file) => {
                let mut data = String::new();
                file.read_to_string(&mut data).unwrap();
                doc.documentation_json = Json::from_str(&data).unwrap();

            }
            Err(error) => { 
                panic!("The file: {:?} could not be found", doc.file); 
            }
        };
        Ok(assert_ne!(Null, doc.documentation_json))
    }

    fn read_type(&self, type_of: String) -> () {
        let mut value = *self.documentation_json.find_path(&["type"]).unwrap();

        if (value.as_string().unwrap() == "documentation".to_string()) {
            self.documentation_json = FuncType {
                *self.documentation_json.find_path(&["value", ])
            }
        }
    }
}
