fn main() {
    // IF
    let number = 3;

    // Parantheses are optional and highly disencouraged
    // The condition must be a boolean
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    // Since `if` is an expression, it can be assigned to some variable
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    // LOOPS

    // loop
    loop {
        println!("again!");
        break;
    }

    // while
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }

    println!("LIFTOFF!!!");

    // for
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // Notice the `Range` notation
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
