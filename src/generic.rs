
#[derive(Debug)]
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


impl<T: std::fmt::Debug> Points<T>{
    fn get_points_2(&self){
        println!("{:#?}", self.points)
    }
}

enum Fruit<T>{
    Banana,
    Apple(T)
}
fn main(){

    let player_1 = Points{
        name: String::from("anton"),
        points: 100
    };

    let fruit_1 = Fruit::Apple("Red");
    let apple_2 = Fruit::Apple(String::from("Green"));
    let apple_3 = Fruit::Apple("Red1".to_string());

    let color = String::from("Red");
 
    let apple_4 = Fruit::Apple(&color);

    let banana:Fruit<String> = Fruit::Banana;

    let a = gen_tup(1,2);
    
    gen_tup(String::from("anton"), 32);

}




fn gen_tup<T,U>(a: T, b:U)->(T,U){
    (a,b)
}