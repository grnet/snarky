use std;
use std::fmt;

#[derive(Debug, Clone)]
pub struct SnarkyError {
    pub message: String,
    pub cause: String,
    pub file: String,
    pub line: u32,
    pub code: u32,
}

impl SnarkyError {
    pub fn create(message: &str, cause: &str, file: &str, line: u32, code: u32) -> Self {
        Self { 
            message: message.to_string(), 
            cause: cause.to_string(),
            file: file.to_string(), 
            line,
            code,
        }
    }
}

impl fmt::Display for SnarkyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "SnarkyError[{}] ({}:{}): {}: {}", 
            self.code,
            self.file, 
            self.line, 
            self.message,
            self.cause,
        )
    }
}
