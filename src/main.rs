//school management system

#[derive(Debug)]

enum Grades{
    A,
    B,
    C,
    D,
    E,
    F
}

#[derive(Debug)]

enum Courses{
    English{score:u8, grade:Grades},
    Maths{score:u8, grade:Grades},
    Science{score:u8, grade:Grades},
}


#[derive(Debug)]
struct Student {
    id: u8,
    name: String,
    class: u8,
    result: Courses
}


impl Student {
    fn new(id: u8, name: String, class: u8, result: Courses) -> Student {
        Self{
            id,
            name,
            class,
            result,
        }
    }

    fn add_students
}



fn main(){
let students: Vec<Students> = vec::new();

}