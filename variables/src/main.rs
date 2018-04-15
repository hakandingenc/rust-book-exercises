fn main() {
    let mut x = 5;
    // Variables are immutable by default
    // mut must be between `let` and `x`

    const MAX_POINTS: u32 = 100_000;
    // For constants, the type must be speficied and value must be known at compile time, thus forbidding function calls (excluding struct and enum constructors)
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let x = 5;
    let x = x + 1; // Not a reassignment, new binding occurrence with shadowing

    let mut spaces = "   ";
    spaces = spaces.len(); //Mutating a variable's type is forbidden
}
