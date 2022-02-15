use crate::dir_browser;
use dir_browser::Directory;

fn format_size(size: u64) -> String {
    if size < 10000 {
        return size.to_string();
    }
    if size > 1000000 {
        let u_size = size / 1024;
        return format!("{} Kb", u_size.to_string());
    }
    let u_size = size / (1024 * 1024);
    format!("{} Mb", u_size.to_string())
}

pub fn format_directory(directory: &Directory) -> String {
    let dir_iter = directory.child_directories.iter();
    let mut dirs: String = String::new();
    for c_d in dir_iter {
        dirs = dirs + &c_d.directory_name;
        dirs.push_str("\n");
    }
    let mut ret = format!(
        "{}:{}\nChild directories\n{}",
        directory.directory_name,
        format_size(directory.size),
        dirs
    );
    ret.to_owned()
}
