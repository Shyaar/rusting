//school management system

use std::{collections::VecDeque, result};

#[derive(Debug)]

enum Grades {
    A,
    B,
    C,
    D,
    E,
    F,
}

impl Grades {
    fn get_grades(score:u8)->Grades{
        match score{
           (0..=44) => Grades::F,
           (45..=54) => Grades::E,
           (55..=64) => Grades::D,
           (65..=74) => Grades::C,
           (75..=84) => Grades::B,
           (85..=100) => Grades::A,
           _=> panic!("Invalid score"),
        }
    }
}

#[derive(Debug)]

enum Courses {
    English { score: u8, grade: Grades },
    Maths { score: u8, grade: Grades },
    Science { score: u8, grade: Grades },
}

#[derive(Debug)]
struct Student {
    id: u8,
    name: String,
    level: u8,
    result: Courses,
}

impl Student {
    fn new(id: u8, name: String, level: u8, result: Courses) -> Student {
        Self {
            id,
            name,
            level,
            result,
        }
    }
}

fn main() {
    

    
}


fn add_student(name:String, level:u8, english_score:u8, maths_score:u8, science_score:u8){
    let students: Vec<Student> = Vec::new();
    let student_id:u8 = 0;
    student_id+=1;

    let english_grade = Grades::get_grades(english_score);
    let maths_grade = Grades::get_grades(maths_score);
    let science_grade = Grades::get_grades(science_score);

    students.push(Student.new(student_id, name, level, result:(Courses::English {score:english_score, grade:english_grade}, Courses::Maths {score:maths_score, grade:maths_grade}, Courses::Science {score:science_score, grade:science_grade})));
}