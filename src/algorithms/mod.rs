use crate::student::Student;

pub enum Choices {
    ONECOMMUTER,
    ONENOTCOMMUTER,
    TWOCOMMUTERS,
    TWONONCOMMUTERS,
}

pub fn choosing(/*choice: Choices, */class: &Vec<Student>) -> Vec<Student> {
    let mut commuters: Vec<Student> = Vec::new();
    let mut non_commuters: Vec<Student> = Vec::new();

    for student in class {
        if student.is_commuting {
            commuters.push(student.clone())
        } else {
            non_commuters.push(student.clone())
        }
    }

    println!("Commuters: ");
    for student in commuters {
        println!("{}", student.name)
    }
    println!("\n");
    
    println!("Non Commuters: ");
    for student in non_commuters {
        println!("{}", student.name)
    }
/* 
    match choice {
        Choices::ONECOMMUTER => {

        }
        Choices::ONENOTCOMMUTER => {

        }
        Choices::TWOCOMMUTERS => {

        }
        Choices::TWONONCOMMUTERS => {

        }
    }
*/    
    todo!()
}