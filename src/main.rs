use std::env;
use std::fs;
use std::path::Path;
use std::vec::Vec;

mod dir_browser;
mod dir_printer;
mod dir_struct;

use dir_browser::browse_root_dir;
use dir_printer::format_directory;
use dir_printer::format_sub_directory;

fn main() {
    let args: Vec<String> = env::args().collect();
    let root_dir = &args[1];

    let p = Path::new(root_dir);
    let dir = browse_root_dir(p);
    if args.len() > 2 {
        let hier = &args[2];
        if let Some(mut dd) = dir {
            println!("{}", format_sub_directory(&mut dd, hier));
        }
    } else {
        if let Some(mut dd) = dir {
            println!("{}", format_directory(&mut dd));
        }
    }
}

pub fn slurp(filename: &String) -> String {
    let txt = fs::read_to_string(filename).expect("No text");
    txt
}
