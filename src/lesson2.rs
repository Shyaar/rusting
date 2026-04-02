#[derive(Debug)]

enum Movies {
    Children(i8),
    Adult(i8),
}

fn main() {
    let your_age = Movies::Children(18);

    if let Movies::Children(age) = your_age {
        if age <=17{
        println!("You can watch this movie, adults only, {age} is too young")
    }
    }else{
        println!("You can watch this movie, adults only")
    }

    let adult = Movies::Adult(22);

    let Movies::Adult(age) = adult else{
        println!("You can't watch this movie, adults only");
        return;
    };

    println!("You can watch this movie, adults only, {age} is old enough")
}
