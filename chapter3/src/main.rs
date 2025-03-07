fn main() {
    let number = 3;

    if number == 3 {
        println!("number was three");
    }

    // let condition = true;
    // let number = if condition {
    //     5
    // } else {
    //     println!("condition was false");
    // };

    // loop {
    //     println!("again!");
    //     break;
    // }

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
    // while 문은 for문보다 느린데, 그 이유는
    // 반복문을 통해 반복될 때 마다 요소에 대한 조건 검사를 수행하는 런타임 코드를 추가하기 때문이다.
    // 반면 for문은 컴파일러가 반복을 최적화하기 때문에 더 빠르다.
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // Range는 한 숫자에서 다른 숫자 전까지 모든 숫자를 차례로 생성.
    // rev() 메서드를 사용하면 역순으로 생성.
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}
