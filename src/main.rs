

struct Points<T>{
    name: String,
    points: T
}

impl Points<String>{
    fn get_name(&self){
        println!("{}", self.name)
    }
}


impl Points<u8> {
    fn get_points(&self){
        println!("{}", self.points)
    }
}

impl<T> Points<T>{
    fn get_points(&self){
        println!("{}", self.points)
    }
}
fn main(){

    let player_1 = Points{
        name: String::from("anton"),
        points: 100
    };

    let a = gen_tup(1,2);
    
    gen_tup(String::from("anton"), 32);

}

fn gen_tup<T,U>(a: T, b:U)->(T,U){
    (a,b)
}