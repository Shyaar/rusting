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



#[derive(Debug)]
enum Tier{
    Gold,
    Silver,
    Platinum,
}

#[derive(Debug)]
enum Subscription{
    Free,
    Basic(f64, u32),
    Premium{tier: Tier}
}

impl Subscription{
    fn summarize(&self){
        match self{
            Subscription::Free => println!("You have limited access to the site"),

            Subscription::Basic(price, duration) => println!("You have limited access to the site's premium features for {price} for {duration} months"),

            Subscription::Premium{tier} => println!("You have full access to the site's premium features. Your tier is {tier:?}"),
        }
    }
}


fn main(){
    let free = Subscription::Free;
    free.summarize();

    let basic = Subscription::Basic(2_500.00, 4);
    basic.summarize();

    let premium = Subscription::Premium{tier: Tier::Platinum};
    premium.summarize();

    
}