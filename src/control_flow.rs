pub fn main() {
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

    // Using `if` in a `let` statement
    let condition = true;
    let number5 = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number5);

    // Repetition with Loops
    // 1. loop
    // 2. while
    // 3. for

    // loop {
    //     println!("again!");
    // }

    // returning value from loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 50 {
            break counter * 2;
        }
    };
    println!("The result is: {}", result);

    // conditional loop with `while` loop
    let mut number6 = 3;

    while number6 != 0 {
        println!("{}", number6);

        number6 -= 1;
    }
    println!("LIFTOFF!!!");

    // Looping Through a Collection with `for` -> Error prone
    let a_for_loop = [10, 20, 30, 40, 50];
    let mut for_loop_index = 0;

    while for_loop_index < 5 {
        println!("The value for loop index is: {}", a_for_loop[for_loop_index]);

        for_loop_index += 1;
    }    

    // for - in -> more concise & safety
    let a_for_in = [10, 20, 30, 40, 50];

    for element in a_for_in.iter() {
        println!("The velue for for-in is: {}", element);
    }

    // Range and Reverse range
    for number7 in (1..4).rev() {
        println!("{}!", number7);
    }
    print!("LIFTOFF!!!");

}
