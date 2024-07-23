#![allow(dead_code, unused_variables)]
mod numbers; // this refers to src/numbers.rs
mod letters; // this refers to src/letters.rs

use letters::words::sentence::response;
use letters::words::Dog;
use numbers::Eight;

fn main() {

    let dog = Dog {};
    let number = Eight {};

    println!("Hello, {dog:?}");
    println!("My number is {number:?}");
    response();
}
