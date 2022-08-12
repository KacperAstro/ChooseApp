use serde_derive::{Deserialize, Serialize};

use crate::json_part::read_json;

pub struct Class {
    students: Vec<Student>,
    class_num: usize,
    students_amount: usize,
    commuting_amount: usize,
    non_commuting_amount: usize,
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
        let students_amount: usize = students.len();
        let mut commuting_amount: usize = 0;

        for student in &students {
            if student.is_commuting {
                commuting_amount += 1;
            }
        }

        let non_commuting_amount = students_amount - commuting_amount;

        Self {
             students,
             class_num, 
             students_amount, 
             commuting_amount, 
             non_commuting_amount 
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