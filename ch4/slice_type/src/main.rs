fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);

    // s.clear(); // can't have a mutable and immutable reference at the same time

    println!("The first word is {word}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
