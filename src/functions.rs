pub fn main() {
    println!("Hello, world!");

    another_function1();

    another_function2(5);

    another_function3(6, 7);

    let a = five();

    println!("The value of a is: {}", a);

    let b = plus_one(8);

    println!("The value of b is: {}", b);
}

fn another_function1() {
    println!("Another function");
}

fn another_function2(x: i32) {
    println!("The value of x is: {}", x);
}

fn another_function3(y: i32, z: i32) {
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
}

fn five() -> i32 {
    5
}

fn plus_one(b: i32) -> i32 {
    b + 1
}
