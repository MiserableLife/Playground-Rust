/*
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}*/

mod front;


pub fn eat_at_restaurant(){
    crate::front::hosting::add_to_waitlist();
    front::hosting::add_to_waitlist();

    use front::hosting;//It's like a symbolic link. make a link 'hosting' to front::hosting here. By default, private.
    hosting::add_to_waitlist();

    use front::hosting::add_to_waitlist as aw;
    aw();


    let order1 = back::Appetizer::Soup;
    let mut meal = back::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    //meal.seasonal_fruit = String::from("blueberries");
}

pub use front::hosting; // re-exporting. external code can see the hosting and access to hosting.  


//root(crate)
// |-- serve_order
// |-- back
//      |--fix_incorrect_order() 
fn serve_order(){
}

mod back{
    pub struct Breakfast{
        pub toast : String,
        seasonal_fruit : String,//private by default
    }

    pub enum Appetizer{//public by default
        Soup,
        Salad,
    }

    impl Breakfast{
        pub fn summer(toast:&str)->Breakfast{
            Breakfast{
                toast:String::from(toast),
                seasonal_fruit:String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order(){
        super::serve_order();
    }
}
