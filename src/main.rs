fn main(){
    let musical_instruments: [String;3]= [
        String::from("guitar"),
        String::from("piano"),
        String::from("drums"),
    ];


    let piano = musical_instruments.get(1);
    println!("{:?}", piano);

    let invalid = musical_instruments.get(100);


    match piano{
        Option::Some(musical_instrument) => println!("{:?}", musical_instrument),
        Option::None => println!("No instrument found"),
    }

    match invalid{
        Option::Some(musical_instrument) => println!("{:?}", musical_instrument),
        Option::None => println!("No instrument found"),
    }
}