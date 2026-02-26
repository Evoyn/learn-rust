fn main() {
    // scalar types
    // integers, floating-point numbers, booleans, characters

    // ! Integers
    // signed integers: i8, i16, i32, i64, i128, isize
    let signed_integer = 10i32;
    println!("Signed integer: {}", signed_integer);

    // unsigned integers: u8, u16, u32, u64, u128, usize
    let unsigned_integer: u32 = 0xff; //hex example
    println!("Unsigned integer: {}", unsigned_integer);
    // isize and usize are platform-dependent
    let isize_value = 10isize;
    let usize_value: usize = 10;
    println!("isize value: {}", isize_value);
    println!("usize value: {}", usize_value);

    // ! Floating-Point Numbers
    // f32 and f64
    let float32: f32 = 3.14f32;
    let float64: f64 = 3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679f64;
    println!("Float32: {}", float32);
    println!("Float64: {}", float64);

    // ! Booleans
    // bool
    let boolean: bool = true;
    println!("Boolean: {}", boolean);

    // ! Characters
    // char
    let character: char = 'a';
    println!("Character: {}", character);

    // ! Compound Types
    // tuples and arrays
    let tuple: (i32, f64, char) = (1, 2.5, 'c');
    println!("Tuple: {:?}", tuple);
    let (first, second, third) = tuple;
    println!("First element: {}", first);
    println!("Second element: {}", second);
    println!("Third element: {}", third);

    let array: [i32; 3] = [1, 2, 3];
    println!("Array: {:?}", array);
    let slice = &array[1..3];
    println!("Slice: {:?}", slice);
}
