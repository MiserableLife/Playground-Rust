#[derive(Debug)]
pub struct Rectangle{
    length : u32,
    width : u32,
}

impl Rectangle {
    pub fn can_hold(&self, other : &Rectangle) -> bool {
        self.length > other.length && self.width > other.width 
    }
    pub fn new(l : i32, w : i32) -> Rectangle { 
        if l < 0 || w < 0 {
            panic!("Rectangel::new has invalid parameter l : {} w : {}", l,w);
        }
        let r = Rectangle{length : l as u32, width : w as u32};
        r

    }
}

pub fn add_two(a : i32) ->i32 { 
    a+2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length : 8, width : 7 };
        let smaller = Rectangle { length : 5, width : 1 } ;

        assert!(larger.can_hold(&smaller));
        /* Adding custom failuer message */
        //assert!(!larger.can_hold(&smaller), "larger rectangle must contain smaller {:?}", larger);
    }
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { length : 8, width : 7 };
        let smaller = Rectangle { length : 5, width : 1 } ;

        //assert!(!smaller.can_hold(&larger));
        assert_eq!(false, smaller.can_hold(&larger));
    }

    #[test] /* an example of should panic attribute */
    #[should_panic]
    fn invalid_rectangle(){
        Rectangle::new(-1,-1);
    }

    #[test]
    #[ignore]
    fn expensive_test(){

    }



}
