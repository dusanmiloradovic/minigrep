use bincode::{deserialize_from, serialize_into, Error};
use dirs::cache_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::fs::{canonicalize, create_dir};
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::path::MAIN_SEPARATOR;
use std::vec::Vec;

#[derive(Serialize, Deserialize)]
pub struct Directory {
    files: Vec<String>,
    pub child_directories: Vec<Box<Directory>>,
    pub(crate) size: u64,
    pub(crate) directory_name: String,
}

fn get_cached_dir(directory: &Path) -> &Path {
    let path_buf = cache_dir().unwrap();
    let cache_path = path_buf.as_path();
    let buf = cache_path.join("minigrep");
    let minigrep_path = buf.as_path();
    if !minigrep_path.exists() {
        create_dir(minigrep_path);
    }
    let root_p = &canonicalize(directory)
        .unwrap()
        .as_path()
        .to_str()
        .unwrap()
        .replace(MAIN_SEPARATOR, "##");
    let buf1 = minigrep_path.join(root_p);
    let cached_dir_path = buf1.as_path();
    cached_dir_path
}

fn read_from_cache(directory: &Path) -> Option<Directory> {
    let cached_dir_path = get_cached_dir(directory);
    if cached_dir_path.exists() {
        let _f = File::open(cached_dir_path);
        if let Ok(f) = _f {
            let r = BufReader::new(f);
            let p: Result<Directory, Error> = deserialize_from(r);
            return match p {
                Ok(d) => Some(d),
                Err(e) => {
                    println!("Error deserializing {}", e);
                    None
                }
            };
        } else {
            let Err(err) = _f;
            println!("Error when reading {}", err);
            None
        }
    } else {
        None
    }
}

fn write_to_cache(directory_path: &Path, directory: &Option<Directory>) {
    let cached_dir_path = get_cached_dir(directory_path);
    let _f = File::create(cached_dir_path);
    if let Ok(f) = _f {
        let bw = BufWriter::new(f);
        serialize_into(bw, &cached_dir_path);
    } else {
        let Err(err) = _f;
        println!("Problem writing to cache {}", err);
    }
}

pub fn browse_root_dir(directory: &Path) -> Option<Directory> {
    match read_from_cache(directory) {
        Some(_directory) => Some(_directory),
        None => {
            let dir = browse_dir(directory);
            write_to_cache(directory, &dir);
            dir
        }
    }
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
        let dir = Directory {
            size: sum_size,
            directory_name,
            files: child_files,
            child_directories,
        };

        Some(dir)
    } else {
        None
    }
}
