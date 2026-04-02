

fn main() {
    println!("{}", user(2));
    println!("{}", user(true));
    println!("{}", user("John"));
    println!("{}", user(String::from("Andrew")));
}

fn user<T>(name: T) -> T {
    name
}
