use crate::garden::vegetables::Asparagus;

pub mod garden; // this refers to src/garden.rs

fn main() {
	let plant = Asparagus {};
	println!("I'm growing {plant:?}");
}