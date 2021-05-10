use std::str::FromStr;

pub fn parse_arg<T: FromStr>(pos: usize, default: &str, message: &str) -> T {
    std::env::args()
        .nth(pos)
        .unwrap_or(default.to_string())
        .parse::<T>()
        .ok()
        .expect(message)
}
