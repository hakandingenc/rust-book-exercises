fn main() {
    another_function(5, 6);
}

// Rust doesn't care about where the function is defined
// Types are required
fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// IMPORTANT DISTINCTION: Statements are instructions that perform some action and do not return a value. Expressions evaluate to a resulting value.

// Putting a semicolon at the end of an expression disables return value.
// The following is thus illegal (compile-time error):
fn some_function() -> i32 {
    let y = {
        let x = 3;
        x + 1; // Doesn't return anything; should omit the semicolon
    }; // Note the semicolon
    y // Error: expected i32, found ()      () := the empty tuple aka void

    // Note `return y`, `return y;`, and `y` are all legal and equivalent.
}
