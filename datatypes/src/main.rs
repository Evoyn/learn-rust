use std::f64::consts::PI;

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
    const PI_VALUE: f64 = PI;
    println!("nilai pi_value: {}", PI_VALUE);
    // format specifier
    println!("nilai pi_value_format: {PI_VALUE:.2}");
    println!("nilai pi_value_format: {0:.2}", PI_VALUE);

    // String
    print!("wwwwwwwww"); //string literal
    print!("wwwwwww,\nqqqqqqq,\"wokwokwok\", C:\\Nebu\\Rust"); //string literal with special character, banyak cari sendiri dah
    let test_raw: &str = r"This is a raw string"; //raw string literal "no special character"
    println!("\n{}", test_raw);

    // Methods
    // method is function that associated with object
    // intinya built in function yang nempel di tipe data tertentu
    let test_method: i8 = -15;
    println!("test method {}", test_method.abs());

    //Casting Types
    // int bs jd float, float bs jd int, int bs jd int lain, float bs jd float lain
    let metercasting: i32 = 1000;
    let meter_i8: u8 = metercasting as u8; //jadi 232 krna melebihi batas i8
    println!("nilai meter_i8: {}", meter_i8);

    //Math Operation
    let tambah: i32 = 5 + 10;
    let kurang: i32 = 95 - 4;
    let kali: i32 = 4 * 30;
    let bagi: f64 = 56.7 / 32.2;
    let modulus: i32 = 43 % 5;
    println!(
        "tambah: {}, kurang: {}, kali: {}, bagi: {:.2}, modulus: {}",
        tambah, kurang, kali, bagi, modulus
    );

    //Augmented Assignment Operations
    let mut augmented: i32 = 10;
    augmented += 5; // augmented = augmented + 5
                    // augmented++ or -- not supported in Rust
    augmented *= 2;
    println!("augmented after += 5: {}", augmented);

    //Bolean Type
    let is_true: bool = true;
    let is_false: bool = false;
    println!("is_true: {}, is_false: {}", is_true, is_false);

    //Bolean Inversion
    let mut is_active: bool = true;
    println!("is_active before inversion: {}", is_active);
    is_active = !is_active; //inversion
    println!("is_active after inversion: {}", is_active);

    //Equality and Comparison Operations
    println!("sama ga {}", "Kue" == "Air");
    println!("beda ga {}", 10 != 5);
    println!("lebih besar {}", 10 > 5);
    println!("sama dengan {}", augmented == 30); // kl 5 == 5 nnti kena warning krna rust ngira ngecompare A == A

    // And, Or Operations
    let inibenar: bool = true;
    let inisalah: bool = false;
    println!("AND operation: {}", inibenar && inisalah); // true jika kedua
    println!("OR operation: {}", inibenar || inisalah); // true jika salah satu
}
