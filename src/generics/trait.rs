use std::fmt::Display;
use std::fmt::Debug;

pub trait Summary{
//    fn summarize(&self) -> String ;
    fn summarize(&self) -> String {
        String::from("Read more...")
    }
}

pub struct NewArticle{
    pub headline : String,
    pub location : String,
    pub author : String,
}
impl Summary for NewArticle {
    fn summarize(&self) -> String {
        format!("{} by {} {}", self.headline, self.author, self.location)
    }
}

pub struct Tweet{
    pub username : String,
}
impl Summary for Tweet{
    fn summarize(&self) -> String{
        format!("{} ", self.username)
    }
}
pub struct Book{
}
impl Summary for Book{
}

pub fn notify(item : impl Summary ) {
//pub fn notify<T : Summary> (item : T ) { // using trait bound is more specific but require more typing
    println!("Breaking news! {} ", item.summarize());
}

/* multiple trait bounds */
pub fn notify_mem(item : impl Summary + Debug){
//pub fn notify_mem<T : Summary + Display>(item : T){    
    println!("Breaking news! {} {:?}", item.summarize(), item);
}

/* using where clause*/
pub fn some_func<T, U >( t: T, u : U) -> i32 
where T : Display + Clone, 
      U : Clone + Debug 
{
    32   
}
fn main()
{
    let tweet = Tweet{
        username : String::from("kdn"),
    };
    println!("1 new tweet {} ", tweet.summarize());

    let book = Book{
    };
    println!("1 new book {}", book.summarize());

    notify(book);

}
