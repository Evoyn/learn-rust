fn main() {
    println!("Hello, world!");
    ini_test();
    ini_test_parameter("Hello from parameter!");
    let result: i32 = square(5);
    println!("The square of 5 is: {}", result);
    println!("The square of 10 is: {}", square(10));
    let unit_value: () = unit_example();
    println!("Unit value: {:?}", unit_value);

    //Block within a function (isolated scope)
    {
        let x = 10;
        let y = 20;
        println!("Inside block: x = {}, y = {}", x, y);
    }

    let block = {
        let a = 5;
        let b = 15;
        a + b // The value of the block is the last expression
    };
    println!("The result of the block is: {}", block);
}

// This function is defined after its usage in main.
// Rust allows this because functions are hoisted.
// The function can be called before its definition.
fn ini_test() {
    println!("This is a test function.");
}

fn ini_test_parameter(param: &str) {
    println!("Parameter received: {}", param);
}

//explicit return value
fn square(num: i32) -> i32 {
    num * num // implicit return without semicolon and return
}

// Unit (empty tuple)
// In Rust, the unit type is represented by an empty tuple `()`. It is used when a function does not return any meaningful value.
fn unit_example() {}
