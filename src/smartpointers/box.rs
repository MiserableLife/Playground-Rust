enum List{
    Cons(i32, Box<List>),
    Nil,
}


use List::{Cons,Nil};


struct MyBox<T>(T);
impl <T> MyBox<T>{
    fn new(x: T) ->MyBox<T> { 
        MyBox(x)
    }
}

use std::ops::Deref;

impl <T> Deref for MyBox<T>{
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}
fn hello(name : &str) { 
    println!("Hello {}!", name);
}


struct CustomSmartPointer{
    data : String 
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self){
        println!("Dropping!!! `{}` ", self.data);
    }
}


fn main(){

    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));


    let x=5;
    let y=&x;
    let z = Box::new(5);

    let myb = MyBox::new(5);

    println!("{}", x==*y);
    println!("{}", x==*z);
    println!("{}", x == *myb);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);//deref coercion converts &m => &String , repeatly &String=> &str. so we can call hello with &m directly .


    let csp = CustomSmartPointer{data : String::from("mystuff")};

   // let cspe = csp;
    //csp.drop(); compiler disallow manually call the drop from Drop traits
    drop(csp);

    println!("End of main function");


}
