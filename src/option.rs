fn main() {
    let musical_instruments: [String; 3] = [
        String::from("guitar"),
        String::from("piano"),
        String::from("drums"),
    ];

    let piano = musical_instruments.get(1);
    println!("{:?}", piano);

    // let invalid = musical_instruments.get(100);

    // match piano {
    //     Option::Some(musical_instrument) => println!("{:?}", musical_instrument),
    //     Option::None => println!("No instrument found"),
    // }

    // match invalid {
    //     Option::Some(musical_instrument) => println!("{:?}", musical_instrument),
    //     Option::None => println!("No instrument found"),
    // }

    play(piano);

    let available: Option <bool> = is_in_stock(true,true);
    println!("{:?}", available);

    match available{
        Some(true) => println!("item is Available"),
        Some(false) => println!("item is not Available"),
        None => println!("item is not in the system"),
    }

    let present = Some(13);
    let missing:Option<i32> = None;

    println!("{:?}", present.unwrap_or(0));
    println!("{:?}", missing.unwrap_or(0));

}

fn play(instrument: Option<&String>) {
    match instrument {
        Option::Some(inst) => println!("playing {inst}"),
        Option::None => println!("No instrument found"),
    }
}


fn is_in_stock(in_system:bool, in_stock:bool)->Option<bool>{
    if in_system && in_stock{
        Some(true)
    }else if in_system{
        Some(false)
    }else{
        None
    }
}