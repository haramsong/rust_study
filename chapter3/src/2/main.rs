fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);
    // Length	Signed	Unsigned
    // 8-bit	i8	    u8
    // 16-bit	i16	    u16
    // 32-bit	i32	    u32
    // 64-bit	i64	    u64
    // arch	    isize	usize

    //   Number literals	Example
    //           Decimal	98_222
    //               Hex	0xff
    //             Octal	0o77
    //            Binary	0b1111_0000
    //    Byte (u8 only)	b'A'

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("The value of b is: {}", b);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
}
