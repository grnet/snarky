use std;
use std::fmt;

#[derive(Debug, Clone)]
pub struct PolyError {
    pub message: String,
    pub file: String,
    pub line: u32,
    pub code: u32,
}

impl PolyError {
    pub fn create(message: &str, file: &str, line: u32, code: u32) -> Self {
        Self { 
            message: message.to_string(), 
            file: file.to_string(), 
            line,
            code,
        }
    }
}

impl fmt::Display for PolyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "PolyError[{}] ({}:{}): {}", 
            self.code,
            self.file, 
            self.line, 
            self.message
        )
    }
}
