struct Point<T,U>{
    x : T,
    y : U,
}
/*
impl  Point<i32,f64>{
    fn distance(&self) ->i32 {
        self.x*self.x + self.y*self.y
    }
}*/
fn main()
{
    let i = 5;
    let f = 4.0;
//    println!("{}",i+f);
    /*
    let both_int = Point{x:5,y:10};
    let both_float = Point{x:1.0, y:4.0};
    let int_float = Point{x:5,y:4.0};
    println!("dist : {}", int_float.distance());*/
}
