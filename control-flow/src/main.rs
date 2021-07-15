fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number2 = 7;

    if number2 < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    
    let number3 = 3;

    if number3 != 0 {
        println!("condition was something other than zero");
    }

    // Handling Multiple Conditions with else if
    let number4 = 6;

    if number4 % 4 == 0 {
        println!("number is divisible by 4");
    } else if number4 % 3 == 0 {
        println!("number is divisible by 3");
    } else if number4 % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
