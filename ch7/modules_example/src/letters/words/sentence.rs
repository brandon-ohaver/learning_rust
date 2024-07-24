pub fn response() {
    println!("You are in sentences.rs!");
    let test = crate::numbers::One {};

    println!("Printing the number {test:?} from within the sentences module!"); // This shows that even though the numbers module is private, that a child mod can use another modules child module
}
