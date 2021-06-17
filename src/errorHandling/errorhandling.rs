use std::fs::File;
use std::io::Read;
use std::io;
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {

    /* Handling Recoverable Error*/
//    let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("Failed to open hello.txt");


    /* Unrecoverable Error*/
    let v = vec![1,2,3];
    v[99];//panic occurs.


}
