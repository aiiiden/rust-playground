mod print_hello;

fn main() {
    // execute the printHello function
    print_hello::print_hello();

    
    {
        let x = 5; // let은 변수를 선언하는데 사용한다.
        println!("The value of x is immutable: {x}");

        let mut y = 4; // mut은 수정이 가능하도록 만든다.
        y = 6;
        println!("The value of x is mutable: {y}");

        const PI_NUM: u32 = 3; // 상수
        println!("PI's floor is {PI_NUM}");
    }

    {
        let foo = 5;
        let foo = foo + 1; // 동일한 이름 재사용 가능
        // foo += 1; // foo는 immutable 하기 때문에 이렇게 사용 불가

        {
            let foo = foo * 2; // 12
            println!("The value of inner scope is: {foo}"); 
        }

        println!("The value of foo is: {foo}"); // 6
    }


    {
        let mut bar = 5;
        bar += 1;
        println!("bar is {bar}")
    }

    
}