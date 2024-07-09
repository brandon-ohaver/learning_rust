// This is the most important function `main`
fn main() {
    println!("Hello, world!");

    // uses another_function to print to the screen
    another_function();

    // uses next_function to print the value to the screen
    next_function(5);

    // creates immutable variable x which holds the return value from add_numbers()
    let x = add_numbers(1, 2);

    // prints the value held in x
    println!("The value of 1 and 2 together is {x}");
}

// this is an example of a simple function that takes no parameters
fn another_function() {
    println!("This is from another function.");
}

// this shows a function with a parameter x with type i32
fn next_function(x: i32) {
    println!("The value of x is: {x}");
}
// This function has two parameters: x and y
// This also has a return value of i32
// This also shows that the `return` keyword is not necessary
// Instead, just don't add a semicolon to the end
fn add_numbers(x: i32, y: i32) -> i32 {
    x + y
}
