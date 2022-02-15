use std::env;
use std::fs;
use std::path::Path;
use std::vec::Vec;

mod dir_browser;
mod dir_printer;

use dir_browser::browse_dir;
use dir_printer::format_directory;

fn main() {
    let p = Path::new(".");
    let dir = browse_dir(p);
    if let Some(dd) = dir {
        println!("{}", format_directory(&dd));
    }
}

pub fn slurp(filename: &String) -> String {
    let txt = fs::read_to_string(filename).expect("No text");
    txt
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}
