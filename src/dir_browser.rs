use std::fs;
use std::path::Path;
use std::vec::Vec;

pub struct Directory {
    files: Vec<String>,
    pub child_directories: Vec<Box<Directory>>,
    pub(crate) size: u64,
    pub(crate) directory_name: String,
}

pub fn browse_dir(directory: &Path) -> Option<Directory> {
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

        Some(Directory {
            size: sum_size,
            directory_name,
            files: child_files,
            child_directories,
        })
    } else {
        None
    }
}
