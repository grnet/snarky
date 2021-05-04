mod rand_helper;
mod parser;
mod macros;
mod error;

pub use rand_helper::snarky_rng;
pub use parser::parse_arg;
pub use error::SnarkyError;
