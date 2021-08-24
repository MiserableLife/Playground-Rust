
//map() 은 match statement 를 단순화시키기 위한 chain 방식으로 알려져있다. 
//Option<T> 를 사용하지 않는 일반 함수들을 사용하기 위한 인터페이스임. 
//X.map(일반함수) 의 의미는 일반 함수의 인자로 X 의 Option(Some 혹은 None ) 을 깐 값을 넣어주고 
//일반 함수가 리턴하는 값을 Option 으로 덮어서 리턴함. 
//Option<T> 를 return 하는 함수에 사용하면 Option<Option<T>> 를 만들어버린다. 이에 대한 보완책으로
//and_then 이라는 combinator 가 존재함. (flatmap)
#[derive(Debug)]
enum Food {
    Apple, 
    Carrot,
    Potato,
}

fn main(){
    let x = Some(Food::Apple);
    let y = x.map(|f| Food::Carrot)
             .map(|f| {})
             .map(|f| Food::Apple);
    println!("{:?}",y);


}
