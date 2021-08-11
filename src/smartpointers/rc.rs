#[cfg(RC)]
enum List{
    Cons(i32, Rc<List>),
    Nil,
}
#[cfg(not(RC))]
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


use std::rc::Rc;

#[cfg(RC)]
fn create_cons(){
    let a = Rc::new(Cons(5, Rc::new(Cons(10,Rc::new(Nil)))));
    println!("cnt of a : {}", Rc::strong_count(&a));
    let b= Cons(3, Rc::clone(&a));
    println!("cnt of a : {}", Rc::strong_count(&a));{
    let c = Cons(4, Rc::clone(&a));
    println!("cnt of a : {}", Rc::strong_count(&a));
    }
    println!("after c out of scope : {}", Rc::strong_count(&a));



}
#[cfg(not(RC))]
fn create_cons(){
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));//compile error because b gets the ownership of a 
}


fn main(){
create_cons();
}
