use std::error::Error;

use data::Plugin;
use extract::extract;
use print::print_recent_file;
use regex::Regex;

pub mod data;
pub mod extract;
pub mod log;
pub mod print;
pub mod util;

// Create once, used everywhere
lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r"##(.*)").unwrap();
}

pub fn dispatch(plugin: &Plugin, path: &str) -> Result<(), Box<dyn Error>> {
    if plugin.date.is_some() {
        print_recent_file(plugin, path)
    } else {
        extract(plugin, path)
    }
}
