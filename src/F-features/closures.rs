fn main()
{
    let closure  = |num : u32 | -> u32 { 
        println!("closure called!{}", num);
        num
    };
    closure(33);

}
