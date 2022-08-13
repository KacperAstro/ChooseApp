use crate::student::{Class, Student};

pub struct Algorithm {
    pub classes: Vec<Class>,
    pub the_chosen_ones: Vec<Student>,
    people_amount: usize,
}

impl Algorithm {
    pub fn new() -> Self {
        let mut classes: Vec<Class> = Vec::new();
        for class_num in 1..=4 {
            classes.push(Class::new_class(&format!("classes/class{}.json", class_num), class_num))
        }
        
        let people_amount = classes[0].get_students().len() + classes[1].get_students().len() + classes[2].get_students().len() + classes[3].get_students().len();
        let the_chosen_ones = Vec::new();

        Self {
            classes,
            the_chosen_ones,
            people_amount,
        }
    }

    pub fn choose_students(&mut self, number_of_chosen: usize) {
        let class_1_percent: f32 = (self.classes[0].get_students().len() as f32 / self.people_amount as f32) * number_of_chosen as f32;
        let class_2_percent: f32 = (self.classes[1].get_students().len() as f32 / self.people_amount as f32) * number_of_chosen as f32;
        let class_3_percent: f32 = (self.classes[2].get_students().len() as f32 / self.people_amount as f32) * number_of_chosen as f32;
        let class_4_percent: f32 = (self.classes[3].get_students().len() as f32 / self.people_amount as f32) * number_of_chosen as f32;
        
        let class_1_chosen_amount = class_1_percent.round();
        let class_2_chosen_amount = (class_1_percent + class_2_percent).round() - class_1_chosen_amount;
        let class_3_chosen_amount = (class_1_percent + class_2_percent + class_3_percent).round() - class_1_chosen_amount - class_2_chosen_amount;
        let class_4_chosen_amount = (class_1_percent + class_2_percent + class_3_percent + class_4_percent).round() - class_1_chosen_amount - class_2_chosen_amount - class_3_chosen_amount;
        
        self.classes[0].chosen_amount = class_1_chosen_amount as usize;
        self.classes[1].chosen_amount = class_2_chosen_amount as usize;
        self.classes[2].chosen_amount = class_3_chosen_amount as usize;
        self.classes[3].chosen_amount = class_4_chosen_amount as usize;

        let highest_commuters_percent_class = self.classes.iter().max_by(|class1, class2| class1.commuters_percent.total_cmp(&class2.commuters_percent)).expect("l").get_class_num();

        println!("--------------------------------------" );
        println!("People amount: {}", self.people_amount);
        println!("--------------------------------------");
        println!("Class 1 amount: {}", self.classes[0].get_students().len());
        println!("Class 1 commuters amount: {}", self.classes[0].get_commuters().len());
        println!("Class 1 non commuters amount: {}", self.classes[0].get_non_commuters().len());
        println!("Class 1 chosen amount: {}", self.classes[0].chosen_amount);
        println!("Class 1 commuters percent: {}", self.classes[0].commuters_percent);
        println!("--------------------------------------");
        println!("Class 2 amount: {}", self.classes[1].get_students().len());
        println!("Class 2 commuters amount: {}", self.classes[1].get_commuters().len());
        println!("Class 2 non commuters amount: {}", self.classes[1].get_non_commuters().len());
        println!("Class 2 chosen amount: {}", self.classes[1].chosen_amount);
        println!("Class 2 commuters percent: {}", self.classes[1].commuters_percent);
        println!("--------------------------------------");
        println!("Class 3 amount: {}", self.classes[2].get_students().len());
        println!("Class 3 commuters amount: {}", self.classes[2].get_commuters().len());
        println!("Class 3 non commuters amount: {}", self.classes[2].get_non_commuters().len());
        println!("Class 3 chosen amount: {}", self.classes[2].chosen_amount);
        println!("Class 3 commuters percent: {}", self.classes[2].commuters_percent);
        println!("--------------------------------------");
        println!("Class 4 amount: {}", self.classes[3].get_students().len());
        println!("Class 4 commuters amount: {}", self.classes[3].get_commuters().len());
        println!("Class 4 non commuters amount: {}", self.classes[3].get_non_commuters().len());
        println!("Class 4 chosen amount: {}", self.classes[3].chosen_amount);
        println!("Class 4 commuters percent: {}", self.classes[3].commuters_percent);
        println!("--------------------------------------");
        println!("Highest commuters percent class: {}", highest_commuters_percent_class)
    }
}