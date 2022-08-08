use std::fs;

use crate::{txt_part::read_file, student::Student};

pub fn writing_to_json(filename: &str, path: &str){
    let class: Vec<Student> = read_file(filename.to_string());
    let class = serde_json::to_string_pretty(&class).expect("Can't write to pretty string");
    fs::write(path.to_string(), class).expect("Can't write to file:");
}

pub fn read_json(filename: String) -> Vec<Student> {
    let class: String = fs::read_to_string(filename).expect("Can't read file");
    let class: Vec<Student> = serde_json::from_str(&class).expect("Can't write to student vec");
    class    
}