// ini file level

// Compailer Directives
// #![allow(unused_variables)] //kalo ini global semua yang ada di file kena smua

// Const
const BIJI: i8 = 2;

// Type Alias
type Kilometer = i32;

//[allow(unused_variables)] // bisa ditaruh sini ntar ngefek ke smua isi function
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

    //Constant > immutable by default and must be type annotated, bisa dipake di file level, variable harus huruf BESAR
    println!("BIJI GW ADA {}", BIJI);

    //Type Alias > liat di file level //fungsinya
    let tinggi_gwej: Kilometer = 200;
    let panjang_apadah_gatau: Kilometer = 3000;

    println!(
        "fungsinya biar rapi aja le ini type {}{}",
        tinggi_gwej, panjang_apadah_gatau
    )

    // Compailer Directives
    // #[] < ini syntaxnya
    // #[allow(unused_variables)] //bisa ditaruh diatas function
    // ini mirip //@TS-ignore
    // bisa di file level jg
    // selain allow banyak yg lain, sisanya cari sendiri
}
