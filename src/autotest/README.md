테스트 속성의 함수들을 이용하기 위해서는 다음 커맨드로 빌드 

$ rustc --test code.rs 
or 
$ cargo test


cargo test 는 여러개의 test 함수들이 존재한다면 스레드를 사용해서 병렬적으로 실행된다. 
그렇기 때문에 각각 테스트 끼리의 종속성이 없도록 하는 것이 중요하고 굳이 해야겠다면 
서로 다른 파일에 구현하거나 스레드를 하나만 이용하는 방식이 있다. 
$ cargo test -- --test-threads=1
--test-threads 옵션은 cargo test 를 이용해서 생성되는 바이너리의 옵션. 
--show-output 옵션은 test function 이 실패할때만 표준 출력을 출력해주는데 성공할 때도 출력하게 해주는 옵션

특정 함수만 테스트 하고 싶다면 
$ cargo test test-name 
을 수행하면 된다. 여기서 test-name 은 패턴 매칭이기 때문에 일부 함수들만 테스트하고싶다면 비슷한 이름들로 묶어두는 것이 편리하다. 


너무 시간이 많이 걸리는 테스트같은 경우에는 평상시에 테스트 하지 않도록 attribute(ignore) 를 줄 수도 있다. 
--ignored 옵션은 ignore attribute 를 지니는 테스트들만 실행하는 옵션


테스트는 크게 unit test 와 integration test 로 구분된다. 
unit test : lib.rs 에 기입한 test 들처럼 각 코드 하나하나가 제대로 동작하는 지 확인하는 테스트. 각 파일의  cfg(test) 로 주석된 tests 모듈.
Rust 에서는 private function 이라 할지라도 테스트할 수 있다. ( permission 초월...) 

integration test : 다른 프로그램들이 내 라이브러리를 사용하는 것 마냥 사용하는 테스트.  
$ cargo test --test integration_test 

특정 integration test 만 실행하고 싶다면 위의 명령어로 테스트할 수 있다. 

integration test 끼리 공유할만한 helper function 들이 필요하다면 
tests/common/mod.rs 
위치에 파일을 생성해서 라이브러리를 만들면 됨. 이는 이전에 배웠던 library module 구조와 관련있음. 


