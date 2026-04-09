fn main(){
    let musical_instruments: [String;3]= [
        String::from("guitar"),
        String::from("piano"),
        String::from("drums"),
    ];


    let piano = musical_instruments.get(1);
    println!("{:?}", piano);

    let invalid = musical_instruments.get(100);

    let valid = piano.expect("unreachable instrument");
    println!("{:?}", valid);

    let invalid = invalid.expect("unreachable instrument");
    println!("{:?}", invalid);
}