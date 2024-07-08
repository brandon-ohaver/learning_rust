fn main() {

    let x = 5; // immutable variable

    let mut y = 6; // mutable variable

    // example of shadowing
    let z = 1; // over shadowed by the redeclared variable of z
    let z = z + 1; // over shadows original declaration

    println!("The value is {z}"); // This will print the non-shadowed z, so this will print 2
}
