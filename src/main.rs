fn main(){
    test();
}
#[derive(Debug, Copy, Clone)]
enum My_Option<T>{
    Some(T),
    None,
}

impl<T> My_Option<T> {
    fn unwarp(self)->T{
        match self{
            My_Option::Some(val)=> val,
            My_Option::None=> panic!("called `My_Option::unwarp()` on a `None` value"),
        }
    }
}

fn test(){
    let a = My_Option::Some(10);
    let b:My_Option<i32> = My_Option::None;
    println!("{} {:?}", a.unwarp(), a);
    println!("{:?}", b);
}