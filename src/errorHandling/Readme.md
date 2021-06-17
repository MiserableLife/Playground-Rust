Rust 는 exception 을 제공하지 않음. 

error 를 recoverable / unrecoverable error 로 구분함. 

panic! 매크로를 사용하면 failure message 를 나타내고 unwind , 스택정리, 종료를 순차적으로 진행함. 
이러한 에러를 unrecoverable error 라고 함. 

recoverable error 는 보통 enum Result<T,E> 를 다룸. 
enum Result<T,E>{
Ok(T),
Err(E),
}
로 정의되어 있음. 이를 이용하여  caller 측에서 Result<T,E> 를 받고서 panic 을 발생시킬지, 예외처리를 할 것인지를 판단하게 된다. 
보통 성공했을 때 T 의 값을 가져오고 에러 시에 처리로직을 작성하는데 match/if 문으로 처리하기 번거롭다.  
let v = Result.expect("message"); / let v = Result.unwrap(); 
성공 시 T 값을 가져오고 에러 시 패닉 발생을 시키고자 한다면, 위의 방식을 이용하는 것이 일반적. 
성공 시 T 값을 가져오고 에러 시 E 를 return 하고자 한다면(error propagation) , 아래와 같은 방식을 사용하는 것이 일반적. 
let s = Result?;


