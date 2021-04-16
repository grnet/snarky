#[macro_export]
macro_rules! map {
    ($($key:expr => $value:expr), +) => {
        {
            let mut map = ::std::collections::HashMap::new();
            $(map.insert($key, $value);)+
            map
        }
    }
}

#[macro_export]
macro_rules! set {
    ($($elem:expr), *) => {
        {
            let mut set = ::std::collections::HashSet::new();
            $(set.insert($elem);)*
            set
        }
    }
}

pub fn parse_arg(pos: usize, default: &str, message: &str) -> usize {
    std::env::args()
        .nth(pos)
        .unwrap_or(default.to_string())
        .parse::<usize>()
        .ok()
        .expect(message)
}


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
