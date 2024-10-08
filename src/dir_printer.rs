use dir_struct::Directory;

use crate::dir_struct;

fn format_size(size: u64) -> String {
    if size < 10000 {
        return size.to_string();
    }
    if size < 1000000 {
        let u_size = size / 1024;
        return format!("{} Kb", u_size.to_string());
    }
    let u_size = size / (1024 * 1024);
    format!("{} Mb", u_size.to_string())
}

pub fn format_directory(directory: &Directory) -> String {
    let mut child_dirs: Vec<&Box<Directory>> = vec![];
    for d in directory.child_directories.iter() {
        child_dirs.push(d);
    }
    child_dirs.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let dir_iter = child_dirs.iter();
    let mut dirs: String = String::new();
    for c_d in dir_iter {
        dirs = dirs + &c_d.directory_name + "\t" + format_size(c_d.size).as_str();
        dirs.push_str("\n");
    }
    let ret = format!(
        "{}:{}\nChild directories\n{}",
        directory.directory_name,
        format_size(directory.size),
        dirs
    );
    ret.to_owned()
}

pub fn format_sub_directory(directory: &Directory, path: &str) -> String {
    //the idea is to check the sizes of arbitary subdirectory, the path is
    // subdir1/subdir2/....

    let dirs: Vec<&str> = path.split("/").collect();
    let mut curr_dir: &Directory = directory;
    for d in dirs {
        let filtered: Vec<&Box<Directory>> = curr_dir
            .child_directories
            .iter()
            .filter(|dr| dr.directory_name == d)
            .collect();
        if filtered.is_empty() {
            return "error".to_string();
        } else {
            curr_dir = &(*(filtered[0]));
        }
    }
    format_directory(curr_dir)
}
