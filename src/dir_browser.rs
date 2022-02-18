use std::fs;
use std::fs::create_dir;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::MAIN_SEPARATOR;
use std::path::{Path, PathBuf};
use std::vec::Vec;

use dirs::cache_dir;
use dunce::canonicalize;
use serde_json::{from_str, to_string, Error};

use dir_struct::Directory;

use crate::dir_struct;

fn get_cached_dir(directory: &Path) -> PathBuf {
    let path_buf = cache_dir().unwrap();
    let cache_path = path_buf.as_path();
    let buf = cache_path.join("minigrep");
    let minigrep_path = buf.as_path();
    if !minigrep_path.exists() {
        create_dir(minigrep_path);
    }
    println!(
        "!!!!*******************************{}",
        String::from(canonicalize(minigrep_path).unwrap().to_string_lossy()),
    );
    let root_p = &canonicalize(directory)
        .unwrap()
        .as_path()
        .to_str()
        .unwrap()
        .replace(MAIN_SEPARATOR, "##")
        .replace(":", "_");
    let buf1 = minigrep_path.join(root_p);
    // println!(
    //     "!!!!????{}",
    //     String::from(canonicalize(&buf1).unwrap().to_string_lossy()),
    // );
    buf1
}

fn read_from_cache(directory: &Path) -> Option<Directory> {
    let cached_dir = get_cached_dir(directory);
    let cached_dir_path = cached_dir.as_path();
    if cached_dir_path.exists() {
        let _f = File::open(cached_dir_path);
        match _f {
            Ok(f) => {
                let mut r = BufReader::new(f);
                let mut data = String::new();
                r.read_to_string(&mut data).expect("Unable to read string");
                let p: Result<Directory, Error> = from_str(data.as_str());
                return match p {
                    Ok(d) => Some(d),
                    Err(e) => {
                        println!("Error deserializing {}", e);
                        None
                    }
                };
            }
            Err(err) => {
                println!("Error when reading {}", err);
                None
            }
        }
    } else {
        None
    }
}

fn write_to_cache(directory_path: &Path, directory: &Option<Directory>) {
    let cached_dir = get_cached_dir(directory_path);
    let cached_dir_path = cached_dir.as_path();
    println!(
        "Creating the cache file {}",
        String::from(cached_dir_path.to_string_lossy())
    );
    let _f = File::create(cached_dir_path);
    match _f {
        Ok(f) => {
            let mut bw = BufWriter::new(f);

            match to_string(directory) {
                Err(err) => {
                    println!("Problem serializing to cache file {}", err);
                }
                Ok(vec) => {
                    bw.write_all(vec.as_bytes());
                    bw.flush();
                }
            }
        }
        Err(err) => {
            println!("Problem writing to cache {}", err);
        }
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
