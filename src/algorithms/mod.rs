use crate::student::{Class};

pub struct Algorithm {
    pub classes: Vec<Class>,
}

impl Algorithm {
    pub fn new() -> Self {
        let mut classes: Vec<Class> = Vec::new();
        for class_num in 1..=4 {
            classes.push(Class::new_class(&format!("classes/class{}.json", class_num), class_num))
        }
        
        Self {
            classes
        }
    }
}