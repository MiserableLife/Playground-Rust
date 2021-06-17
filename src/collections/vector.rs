fn main(){
    let vv = vec![1,2,3];
    let vvv : Vec<i32> = Vec::new();
    let mut v = Vec::new();
    v.push(5);
    v.push(6);

    let e : &i32 = &vv[2];
    println!("The third element {}", e);

    match vv.get(2){
        Some(third) => println!("The third element {} ", third), 
        None => println!("There is no third element"),
    }

    for i in &v {
        println!("{} in v ", i);
    }
    for i in &v {
        i+=50;
    }

}
