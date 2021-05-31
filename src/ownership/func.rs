
fn main()
{
    let s = String::from("hello");
    takes_ownership(s);
    let x = 5;
    makes_copy(x);

    /*s.len()*/
}

fn takes_ownership(some_string : String)
{
    println!("{}", some_string);
}

fn makes_copy(some_integer : i32)
{
    println!("{}", some_integer);
}

fn calculate_len(s : String) -> (String, usize) { 

    let len = s.len(); 
    (s, length)
}
