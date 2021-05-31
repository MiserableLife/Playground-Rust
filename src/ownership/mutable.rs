fn main() {

    let mut s = String::from("hello");
 //   let r3 = &mut s;
    let r3 = & s;
    println!("{} {}", s, r3);

}
