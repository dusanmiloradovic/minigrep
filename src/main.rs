use std::env;
use std::fs;
use std::path::Path;
use std::vec::Vec;

fn main() {
    let p = Path::new(".");
    println!("{:?}", p.is_relative());
    println!("{:?}", p.parent());
    browse_dir(p);
}

struct Directory {
    files: Vec<String>,
    child_directories: Vec<Box<Directory>>,
    size: u64,
    directory_name: String,
}

fn browse_dir(directory: &Path) -> Option<Directory> {
    let d = fs::read_dir(directory);
    if let Ok(rez) = d {

        let mut sum_size: u64 = 0;
        let mut child_directories: Vec<Box<Directory>> = Vec::new();
        let mut child_files: Vec<String> = Vec::new();
        let mut directory_name: String = String::new();
        if let Some(f_name) = directory.file_name() {
            if let Some(f_name_str) = f_name.to_str() {
                directory_name = f_name_str.to_string();
            }
        };
        rez.for_each(|f| {
            // println!("{:?}", f);
            if let Ok(_f) = f {
                let f_path = _f.path();


                if let Ok(file_type) = _f.file_type() {
                    if file_type.is_dir() {
                        if let Some(dir) = browse_dir(&f_path) {
                            sum_size = sum_size + dir.size;
                            let boxed_dir = Box::from(dir);
                            child_directories.push(boxed_dir);
                        }
                    } else {
                        if let Ok(metadata) = _f.metadata() {
                            sum_size = sum_size + metadata.len();
                            if let Some(f_name) = _f.path().as_path().file_name() {
                                if let Some(f_name_str) = f_name.to_str() {
                                    child_files.push(f_name_str.to_string());
                                }
                            }
                        }
                    }
                } else {
                    println!("Couldn't get file type for {:?}", _f.path());
                }
            }
        });
        let h = sum_size / (1024 * 1024);
        Some(Directory { size: sum_size, directory_name, files: child_files, child_directories })
    } else {
        None
    }
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
