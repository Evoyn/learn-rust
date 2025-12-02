fn main() {
    let apples: i32 = 50; // variable Immutable by default
    let mut oranges: i32 = 20; // add mut for mutable variable
    let _fruit: i32 = apples + oranges; // _ remove warning

    println!(
        "I have {1} oranges and {0} apples, yay {0} apples",
        apples, oranges
    );
    // OR
    oranges = 30;
    println!("I have {oranges} oranges"); // new Syntax

    //Variable Shadowing
    let grams_of_proteins: &str = "100.345";
    println!("String {}", grams_of_proteins);
    let grams_of_proteins: f64 = 100.345;
    println!("Float {}", grams_of_proteins);
    let mut grams_of_proteins: i64 = 100;
    println!("Int {}", grams_of_proteins);
    grams_of_proteins = 50;
    println!("Int 0{}", grams_of_proteins);

    //Scope
    {
        println!("ini jeruk ada {}", oranges) // bisa akses dari luar scope, tapi yang diluar gabisa akses yg dalem
    }
}
