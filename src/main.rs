//school management system


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
    result: Vec<Courses>,
}

impl Student {
    fn new(id: u8, name: String, level: u8, result: Vec<Courses>) -> Student {
        Self {
            id,
            name,
            level,
            result,
        }
    }
}

fn main() {
    let mut students:Vec<Student> = vec![];
    add_student(String::from("john"), 100, &mut students);

    get_all_students(&mut students);

    



}


fn get_all_students(students:&mut Vec<Student>){
    println!("{:#?}", students)
}
fn add_student(name:String, level:u8, students:&mut Vec<Student>){
    let mut student_id:u8 = 0;
    student_id+=1;
    
    let result = vec![
        Courses::English { score: 0, grade: Grades::F },
        Courses::Maths { score: 0, grade: Grades::F },
        Courses::Science { score: 0, grade: Grades::F },
    ];

    students.push(Student::new(student_id, name, level,result));
}

fn update_scores(students: &mut  )