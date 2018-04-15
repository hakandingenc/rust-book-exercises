fn main() {
    // SCALAR TYPES

    let a = 0b1111_0000u8;
    // Integer literals take a form prefix, an optional type suffix (8, 16, 32, 64 and u or i), and `_` as a visual separator
    let b = b'A';
    // Byte literal doesn't allow type suffix (mandatorily u8)
    // Note that Rust defaults to i32 for integer literals

    let x = 2.0; // Rust defaults to f64 for floating-point types
    let y: f32 = 3.0;

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻'; // UTF-8 includes emojis

    //COMPOUND TYPES (tuples and arrays)

    let tup: (i32, f64, u8) = (500, 6.4, 1); // Types are optional when allowed by context

    // Tuple pattern matching:
    let (x, y, z) = tup;

    let a = [1, 2, 3, 4, 5];
    // Note that an out-of-bounds array access is a runtime error in Rust
}
