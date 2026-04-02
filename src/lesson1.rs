#[derive(Debug)]

enum Roster {
    Onduty,
    Offduty,
    Sick,
    Vacation,
    OfficialDuty,
    UnpaidLeave,
}

impl Roster {
    fn check(&self) {
        match self {
            Roster::Onduty => println!("On duty"),

            Roster::Offduty | Roster::Vacation => {
                println!("Staff is not working at the moment");
            }

            Roster::Sick => {
                println!("Staff is sick");
            }

            _other_status => {
                println!("Staff is {_other_status :?}")
            }
        }
    }
}

#[derive(Debug)]
enum Shoe{
    Size(i32),
    Brand(Brands)
}

#[derive(Debug)]
enum Brands{
    Nike,
    Adidas,
    Puma,
    Reebok,
}

impl Shoe {
    fn size(&self){
        match self {
            Shoe::Size(0..=39) =>{
                println!("Them some small ass shoes boy")
            },
            Shoe::Brand(Brands::Nike) =>{
                println!("Nike")
            },
            Shoe::Brand(Brands::Adidas) =>{
                println!("Adidas")
            },
            Shoe::Brand(Brands::Puma) =>{
                println!("Puma")
            },
            Shoe::Brand(Brands::Reebok) =>{
                println!("Reebok")
            },
            Shoe::Size(size) =>{
                println!("oooh {size}!!!!!!!, Them some big boy feet")
            }
        }
    }
}

fn main() {
    let status = Roster::UnpaidLeave;
    status.check();

    let shoe = Shoe::Size(29);
    shoe.size();

}
