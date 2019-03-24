#![allow(unused_variables)]
fn main() {
    //// Section 3.1 ////
    // Mutable variables
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Shadowing (most recent variable takes precedence)
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    // Shadowing allows for change of type on re-assignment
    let spaces = "   ";
    let spaces = spaces.len();


    //// Section 3.2 ////
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    let t = true;

    let f: bool = false; // with explicit type annotation

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // Arrays
    // memory is allocated on stack
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
