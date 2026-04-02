

fn main() {
    println!("{}", user::<i8>(2));
    println!("{}", user::<bool>(true));
    println!("{}", user::<&str>("John"));
    println!("{}", user::<String>(String::from("Andrew")));
}

fn user<T>(name: T) -> T {
    name
}
