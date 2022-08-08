use serde_derive::{Deserialize, Serialize};

use crate::json_part::read_json;

#[derive(Clone)]
pub struct Class {
    students: Vec<Student>,
    class_num: usize,
    students_amount: usize,
    commuting_amount: usize,
    non_commuting_amount: usize,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
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

    pub fn get_students(&self) -> Vec<Student> {
        self.students.clone()
    }

    pub fn get_class_num(&self) -> usize {
        self.class_num
    }

    pub fn get_amount(&self) -> usize {
        self.students_amount
    }

    pub fn get_commuting_amount(&self) -> usize {
        self.commuting_amount
    }

    pub fn get_non_commuting_amount(&self) -> usize {
        self.non_commuting_amount
    }
}

impl Student {
    pub fn new(name: String, is_commuting: bool) -> Self {
        Self { name, number_duties: 0., is_commuting }
    }
}