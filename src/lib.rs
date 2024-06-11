use regex::Regex;

pub mod extract;
pub mod log;
pub mod util;

// Create once, used everywhere
lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r"##(.*)").unwrap();
}
