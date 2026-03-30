
#[derive(Debug)]



enum Heater{
    On,
    Off,
}

impl Heater {
    fn set_temp(self, temp:u8)->String{
        match temp {
            0..=20 => String::from ("cold"),
            21..=40 => String::from ("warm"),
            41..=60 => String::from ("lukewarm"),
            61..=80 => String::from ("hot"),
            81..=100 => String::from ("very-hot"),
            _ => String::from ("unknown"),
        }
    }
    fn temp(self, temp:u8){
        let set_temp = self.set_temp(temp);
        println!("the temprature is set to {}",set_temp);
}}

fn main() {
    let bath = Heater::On.temp(52);
}
