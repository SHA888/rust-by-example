use std::io;

fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    println!("x is: {} and y is: {}", x, y);

    // mathematical operations
    // addition
    let sum = 5 + 10;

    // substraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    println!("Sum: {}, difference: {}, product: {}, quotient: {}, remainder: {}", sum, difference, product, quotient, remainder);

    let t = true;

    let f: bool = false; // with explicit type annotation

    println!("t: {}, f: {}", t, f);

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("c: {}, z: {}, heart_eyed_cat: {}", c, z, heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1); // single compound element

    // destructuring
    let (x, y, z) = tup;

    println!("x: {}, y: {}, z: {}", x, y, z);

    // using period index
    let tup2: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = tup2.0;

    let six_point_four = tup2.1;

    let one = tup2.2;

    println!("five_hundred: {}, six_point_four: {}, one: {}", five_hundred, six_point_four, one);

    // Array
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let first_month = months[0];

    println!("First month: {}", first_month);

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Please enter en array index.");

    let mut index = String::new();
    
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    let first = a[0];
    let second = a[1];

    println!("first: {}, second: {}", first, second);
    println!("The value of the element at index {} is: {}", index, element);
}
