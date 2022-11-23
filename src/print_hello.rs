/**
Hello World를 출력하는 함수
- `pub`는 접근지정자. public의 준말로, 외부에서도 접근 가능하다는 의미
- `crate`는 "나무상자"라는 의미로 다른 프로그램에서 사용하기 위한 부분 (독립적으로 실행할 수 없음)
*/
pub(crate) fn print_hello() {
    println!("Hello, world!");
}