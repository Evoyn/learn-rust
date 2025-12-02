fn main() {
    println!("Data Types");

    // scalar type int,float,bool,char

    // Integer
    //signed and unsigned integer
    // i8,u8,i16,u16,i32,u32,i64,u64,i128,u128,isize,usize

    // signed support both positive and negative values "start with i"
    let a: i8 = 127; // max value for i8
    let a2 = 127i8; // another way to declare
    println!("nilai a: {} / {}", a, a2);

    // unsigned only support 0 to positive values "start with u", can store more data in positive range
    let c: u8 = 255; // max value for u8
    let c2 = 255u8; // another way to declare
    println!("nilai c: {} / {}", c, c2);

    // _ as separator for better readability
    let angkanya_banyak: u64 = 1_000_000_000;
    println!("nilai angkanya_banyak: {}", angkanya_banyak);

    //usize and isize depend on architecture (32bit or 64bit)
    // misalnya di 64bit architecture, maka isize = i64 dan usize = u64
    let size: usize = 255;
    println!("nilai size: {}", size);

    // Float
    // f32 (6-9 digit dibelakang ,),f64 (15-17 digit dibelakang ,)

    // String
    print!("wwwwwwwww"); //string literal
    print!("wwwwwww,\nqqqqqqq,\"wokwokwok\", C:\\Nebu\\Rust"); //string literal with special character, banyak cari sendiri dah
    let test_raw: &str = r"This is a raw string"; //raw string literal "no special character"
    println!("\n{}", test_raw);
}
