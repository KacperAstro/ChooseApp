use serde_derive::{Deserialize, Serialize};

pub enum Class {
    Class1,
    Class2,
    Class3,
    Class4,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Student {
    pub name: String,
    pub number_duties: f32,
    pub is_commuting: bool,
}

impl Student {
    pub fn new(name: String, is_commuting: bool) -> Self {
        Self { name, number_duties: 0., is_commuting }
    }
}