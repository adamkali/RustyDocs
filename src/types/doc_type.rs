
/**DocType
 
 # Attributes
 
 * `name` The name of the the documentation given in the chosen documentation.json 
 * `params` This can be a list of strings like 
     [ 
         {
             "fancyName": "foo"
         },
         {
             "stupidName": "Does not explain at all"
         }
     ]
 * `returns` This shows the user what the method is supposed to return.
 * `explanation` This tells the user what the function does.
 * `errors` This tells the user what possible errors that this function can throw .
 */ 
pub struct FuncType {
    name: String,
    params: Vec<String>,
    returns: String,
    explanation: String,
    errors: String,
}

impl FuncType {
    pub fn new() -> FuncType {
        return FuncType {
            name: "[New Doc]".to_string(),
            params: vec!("[New Param]".to_string()),
            returns: "[New Return]".to_string(),
            explanation: "".to_string(),
            errors: "[New Errors]".to_string(),
        }
    }
}