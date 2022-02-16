use std::env;
use std::fs;
use std::path::Path;
use std::vec::Vec;

mod dir_browser;
mod dir_printer;

use dir_browser::browse_dir;
use dir_printer::format_directory;
use dir_printer::format_sub_directory;

fn main() {
    let args: Vec<String> = env::args().collect();
    let root_dir = &args[1];

    let p = Path::new(root_dir);
    let dir = browse_dir(p);
    if (args.len() > 2) {
        let hier = &args[2];
        if let Some(dd) = dir {
            println!("{}", format_sub_directory(&dd, hier));
        }
    } else {
        if let Some(dd) = dir {
            println!("{}", format_directory(&dd));
        }
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
