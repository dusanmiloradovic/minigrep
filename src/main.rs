use std::env;
use std::fs;
use std::path::Path;
use std::vec::Vec;

mod dir_browser;

fn main() {
    let p = Path::new(".");
    println!("{:?}", p.is_relative());
    println!("{:?}", p.parent());
    dir_browser::browse_dir(p);
}


pub fn slurp(filename: &String) -> String {
    let txt = fs::read_to_string(filename).expect("No text");
    txt
}

struct Cacher<T>
    where
        T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}
