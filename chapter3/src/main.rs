fn main() {
    println!("Hello, world!");
    another_function();
    print_number(5);
    print_numbers(5, 6);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn another_function() {
    println!("Another function.");
}

fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn print_numbers(x: i32, y: i32) {
    println!("x is: {}", x);
    println!("y is: {}", y);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
