fn main() {
    println!("Hello, world!");

    another_function(5, 'a');
    let five = five();
    println!("The value of five is: {}", five);
    let five_plus_one = five_plus_one();
    println!("The value of 5 + 1 is: {}", five_plus_one);
}

fn another_function(x: i32, y: char) {
    println!("Another function with parameter {} {}", x, y);
}

fn five() -> i32 {
    5
}

fn five_plus_one() -> i32 {
    five() + 1
}
