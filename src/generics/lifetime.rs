fn longest<'a>(x:  &'a str, y :   &'a str ) -> &'a str{
    if x.len() > y.len() {
        x
    }
    else{
        y
    }
}


struct ImportantExcerpt<'a>{
    part : &'a str,
}


/* Actually, it should write lifetime annotation to be comiled.
 * But, Rust developer found the lifetime could be inferred in a particular situation.
 * So, there is no problem even if we omit lifetime annotation in this function. 
 * Therefore, as time goes, there will be less situation to write lifetime annotation explicitly */
fn first_word(s: &str) -> &str { 
    let bytes = s.as_bytes();
    for(i , &item ) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main()
{
    /* this code will not be compiled because of the wrong usage of lifetime. 
    let string1 = String::from("abcd");
    let result;
    {
    let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
    */

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt{ part : first_sentence } ;
    println!("{}",i.part);


    let s : &'static str="I have a static lifetime";//live entire lifetime of the program
}
