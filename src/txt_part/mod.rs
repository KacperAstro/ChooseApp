use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::student::Student;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_file(filename: String) -> Vec<Student>{
    let mut class1: Vec<Student> = Vec::new();

    match read_lines(filename) {
        Ok(lines) => {
            for line in lines {
                match line {
                    Ok(l) => {
                        let lines: Vec<&str> = l.split(" ").collect();
                        let name = lines[..2].join(" ");
                        let is_commuting: bool = lines[2].parse().unwrap();
                        
                        let student = Student::new(name, is_commuting);
                        class1.push(student);
                    }
                    Err(_) => {
                        println!("Can't read line")
                    }
                }
            }
        }
        Err(_) => {
            println!("Can't read file");
        }
    }
    class1
}
