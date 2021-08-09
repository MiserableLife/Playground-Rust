#[derive(Debug)]
enum Message{
    Quit,
    Move { x:i32, y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message{
    fn call(&self){
        println!("it's me {:?}", self);
    }
    fn matchcall(&self){
        match self { 
            Message::Quit=> println!("Quit!"),
            Message::Move{x,y}=>println!("Move! {} {}",x,y),
            Message::Write(_)=>println!("WRite!"),
            Message::ChangeColor(_,_,_)=>println!("ChangeColor"),
            _ =>println!("not supported!"),
        }
    }
}

fn main()
{
    let m = Message::Write(String::from("hello"));
    let mm = Message::Move{x:10,y:119};
    let mc = Message::ChangeColor(255,255,255);
    m.call();
    mm.call();
    mc.call();


    /*Option*/
    let x  = Some(5);
    let y : Option<i32> = None;
    if let Some(5) = x {
        println!("I guessed x correctly");
    }
    //x + y;
    //
    //

    /*match*/
    m.matchcall();
    mm.matchcall();
    mc.matchcall();


}
