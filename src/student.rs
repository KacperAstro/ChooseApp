use serde_derive::{Deserialize, Serialize};

use crate::json_part::read_json;

pub struct Class {
    students: Vec<Student>,
    class_num: usize,
    pub chosen_amount: usize,
    pub commuters_percent: f32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Student {
    pub name: String,
    pub number_duties: f32,
    pub is_commuting: bool,
}

impl Class {
    pub fn new_class(filename: &str, class_num: usize) -> Self {
        let students = read_json(filename.to_string());
        
        let mut commuters_amount = 0;
        for student in &students {
            if student.is_commuting {
                commuters_amount += 1;
            }
        }
        
        let commuters_percent = commuters_amount as f32 / students.len() as f32;
        let chosen_amount = 0;
        
        Self {
             students,
             class_num,
             chosen_amount,
             commuters_percent, 
        }
    }

    pub fn get_students(&mut self) -> &mut [Student] {
        &mut self.students
    }

    pub fn get_commuters(&mut self) -> Vec<&mut Student> {
        // I personally would have this method return the iterator directly, instead of turning it into a vec first.
        // So, I’d get rid of the collect, and make the method return -> impl Iterator<Item=&mut Student>
        self.get_students().iter_mut().filter(|x|  x.is_commuting).collect()
    }

    pub fn get_non_commuters(&mut self) -> Vec<&mut Student> {
        self.get_students().iter_mut().filter(|x|  x.is_commuting).collect()
    }

    pub fn get_class_num(&self) -> usize {
        self.class_num
    }
}

impl Student {
    pub fn new(name: String, is_commuting: bool) -> Self {
        Self { name, number_duties: 0., is_commuting }
    }
}