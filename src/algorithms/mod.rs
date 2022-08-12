use crate::student::{Class, Student};

pub struct Algorithm {
    pub classes: Vec<Class>,
    pub the_chosen_ones: Vec<Student>,
}

impl Algorithm {
    pub fn new() -> Self {
        let mut classes: Vec<Class> = Vec::new();
        for class_num in 1..=4 {
            classes.push(Class::new_class(&format!("classes/class{}.json", class_num), class_num))
        }
        
        let the_chosen_ones = Vec::new();

        Self {
            classes,
            the_chosen_ones,
        }
    }

    pub fn choose_students(&mut self, number_of_chosen: usize) {
        // CLasses Data
        // #1
        let class_1_amount = self.classes[0].get_students().len();
        let class_1_commuters_amount = self.classes[0].get_commuters().len();
        let class_1_non_commuters_amount = self.classes[0].get_non_commuters().len();
        
        // #2
        let class_2_amount = self.classes[1].get_students().len();
        let class_2_commuters_amount = self.classes[1].get_commuters().len();
        let class_2_non_commuters_amount = self.classes[1].get_non_commuters().len();

        // #3
        let class_3_amount = self.classes[2].get_students().len();
        let class_3_commuters_amount = self.classes[2].get_commuters().len();
        let class_3_non_commuters_amount = self.classes[2].get_non_commuters().len();

        // #4
        let class_4_amount = self.classes[3].get_students().len();
        let class_4_commuters_amount = self.classes[3].get_commuters().len();
        let class_4_non_commuters_amount = self.classes[3].get_non_commuters().len();
        
        // math :D
        let people_amount = class_1_amount + class_2_amount + class_3_amount + class_4_amount;

        let class_1_percent: f32 = (class_1_amount as f32 / people_amount as f32) * number_of_chosen as f32;
        let class_2_percent: f32 = (class_2_amount as f32 / people_amount as f32) * number_of_chosen as f32;
        let class_3_percent: f32 = (class_3_amount as f32 / people_amount as f32) * number_of_chosen as f32;
        let class_4_percent: f32 = (class_4_amount as f32 / people_amount as f32) * number_of_chosen as f32;
        
        let class_1_chosen_amount = class_1_percent.round();
        let class_2_chosen_amount = (class_1_percent + class_2_percent).round() - class_1_chosen_amount;
        let class_3_chosen_amount = (class_1_percent + class_2_percent + class_3_percent).round() - class_1_chosen_amount - class_2_chosen_amount;
        let class_4_chosen_amount = (class_1_percent + class_2_percent + class_3_percent + class_4_percent).round() - class_1_chosen_amount - class_2_chosen_amount - class_3_chosen_amount;
        
        let class_1_commuters_percent: f32 = class_1_commuters_amount as f32 / class_1_amount as f32; 
        let class_2_commuters_percent: f32 = class_2_commuters_amount as f32 / class_2_amount as f32; 
        let class_3_commuters_percent: f32 = class_3_commuters_amount as f32 / class_3_amount as f32; 
        let class_4_commuters_percent: f32 = class_4_commuters_amount as f32 / class_4_amount as f32; 

        println!("--------------------------------------");
        println!("People amount: {}", people_amount);
        println!("--------------------------------------");
        println!("Class 1 amount: {}", class_1_amount);
        println!("Class 1 commuters amount: {}", class_1_commuters_amount);
        println!("Class 1 non commuters amount: {}", class_1_non_commuters_amount);
        println!("Class 1 chosen amount: {}", class_1_chosen_amount);
        println!("Class 1 commuters percent: {}", class_1_commuters_percent);
        println!("--------------------------------------");
        println!("Class 2 amount: {}", class_2_amount);
        println!("Class 2 commuters amount: {}", class_2_commuters_amount);
        println!("Class 2 non commuters amount: {}", class_2_non_commuters_amount);
        println!("Class 2 chosen amount: {}", class_2_chosen_amount);
        println!("Class 2 commuters percent: {}", class_2_commuters_percent);
        println!("--------------------------------------");
        println!("Class 3 amount: {}", class_3_amount);
        println!("Class 3 commuters amount: {}", class_3_commuters_amount);
        println!("Class 3 non commuters amount: {}", class_3_non_commuters_amount);
        println!("Class 3 chosen amount: {}", class_3_chosen_amount);
        println!("Class 3 commuters percent: {}", class_3_commuters_percent);
        println!("--------------------------------------");
        println!("Class 4 amount: {}", class_4_amount);
        println!("Class 4 commuters amount: {}", class_4_commuters_amount);
        println!("Class 4 non commuters amount: {}", class_4_non_commuters_amount);
        println!("Class 4 chosen amount: {}", class_4_chosen_amount);
        println!("Class 4 commuters percent: {}", class_4_commuters_percent);
        println!("--------------------------------------");
    }
}