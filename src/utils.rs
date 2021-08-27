use std::fs;
use std::path::Path;

pub fn to_string(option: Option<&String>, default: &str) -> String {
    option.unwrap_or(&default.to_string()).to_string()
}

pub fn file_extension(filename: &str) -> String {
    Path::new(filename)
        .extension()
        .map(|t| t.to_str().map(|r| ".".to_owned() + &r.to_string()))
        .unwrap_or_default()
        .unwrap_or_else(|| "".to_string())
}

pub fn copy_file(from: String, to: String) -> u64 {
    fs::copy(from.to_string(), to.to_string()).expect(
        &*("Something went wrong copying file: ".to_owned() + from.as_str() + "\n" + to.as_str()),
    )
}

pub fn remove_file(path: String) {
    fs::remove_file(path.to_string())
        .expect(&*("Something went wrong removing file: ".to_owned() + path.as_str()))
}

pub fn remove_directories(path: String) {
    fs::remove_dir_all(path.to_string())
        .expect(&*("Something went wrong removing directories: ".to_owned() + path.as_str()))
}

pub fn read_file(path: String) -> String {
    fs::read_to_string(path.to_string())
        .expect(&*("Something went wrong opening file: ".to_owned() + path.as_str()))
}

pub fn create_file(path: String, contents: String) {
    fs::write(path.to_string(), contents)
        .expect(&*("Something went wrong creating file: ".to_owned() + path.as_str()))
}

pub fn create_directory(path: String) {
    fs::create_dir_all(path.to_string())
        .expect(&*("Something went wrong creating directory: ".to_owned() + path.as_str()))
}
