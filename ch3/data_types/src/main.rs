use std::io;

fn main() {
    // if `u32` type notation is not used, then Rust will require it and error
    let guess: u32 = "42".parse().expect("Not a number!");
    // this is required due to parse() which could infer may different types


    // when inputing a number larger than the given array, an index out of bounds error will be printed to the screen
    let a = [1, 2, 3, 4, 5]; // This is an array

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Should be a number!");

    let index: usize = index.trim().parse().expect("Not a number!");

    let element = a[index];

    println!("The element is {element}");
}
