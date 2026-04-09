use std::string;

fn main(){
    let musical_instruments: [String;3]= [
        String::from("guitar"),
        String::from("piano"),
        String::from("drums"),
    ];


    let piano = musical_instruments.get(100);
    println!("{:?}", piano);
}