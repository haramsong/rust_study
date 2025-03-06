fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    // shadowing
    let spaces = "    ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    // let mut space = "    ";
    // space = space.len();
    // 10 |     let mut space = "    ";
    //    |                     ------ expected due to this value
    // 11 |     space = space.len();
    //    |             ^^^^^^^^^^^ expected `&str`, found `usize`
    // let, mut 차이
    // let: 변수를 선언할 때 값을 바꿀 수 없음, shadowing을 통해 새로운 타입, 새로운 값 할당 가능
    // mut: 변수를 선언할 때 값을 바꿀 수 있음, 타입 고정(변경 불가능)
}
