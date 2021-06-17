fn main()
{
    for c in "김치찌개".chars(){//separate unicode scalar
        println!("{}",c);
    }
    for b in "김치찜".bytes(){
        println!("{}",b);
    }
}
