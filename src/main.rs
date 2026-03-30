#[derive(Debug)]

enum ROLLSTAR {
    Onduty,
    Offduty,
    Sick,
    Vacation,
    Official_Duty,
    Unpaid_Leave,
}

impl ROLLSTAR {
    fn check(&self) {
        match self {
            ROLLSTAR::Onduty => println!("On duty"),

            ROLLSTAR::Offduty | ROLLSTAR::Vacation => {
                println!("Staff is not working at the moment");
            }

            ROLLSTAR::Sick => {
                println!("Staff is sick");
            }

            _other_status => {
                println!("Staff is {_other_status :?}")
            }
        }
    }
}
fn main() {
    let status = ROLLSTAR::Unpaid_Leave;
    status.check();
}
