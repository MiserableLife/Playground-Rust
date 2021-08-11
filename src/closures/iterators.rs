fn main(){
    let v = vec![1,2,3];

    let vi = v.iter();
    for val in vi {
        println!("Got {}", val);
    }

    let vp : Vec<_> = v.iter().map(|x|x+1).collect();
    assert_eq!(vp, vec![2,3,4]);

}
