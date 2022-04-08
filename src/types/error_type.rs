pub struct ErrorType {
    code: String,
    cause: Vec<String>,
    fix: String,
    explanation: String,
}

impl ErrorType {
    pub fn new() -> ErrorType {
        return ErrorType {
            code: "[New Doc]".to_string(),
            cause: vec!("[New Param]".to_string()),
            fix: "[New Return]".to_string(),
            explanation: "".to_string(),
        }
    }
}