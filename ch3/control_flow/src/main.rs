// this shows off how an if expression works
fn main() {
    let number = 3;

    if number < 5 {
        // this prints if the value in number is less than 5 making the expression true
        println!("condition was true");
    } else {
        // this prints if the value in number is greater than or equal to 5 making the expression false
        println!("condition was false")
    }

    // This variable holds the true boolean
    let boolean = true;

    // this if expression will only run if boolean is true
    if boolean {
        println!("boolean is true")
    }

    // this if expression is showing how to use the ! operator
    // this if expression runs as long as number is any number excluding zero
    if number != 0 {
        println!("number is something other than zero")
    }

    let number = 6;

    // this shows how to use else if expressions
    // doing this will cause the second, but not the third statement to print even though they are both true
    // this is because Rust prints the first if expression that evaluates to true
    if number % 4 == 0 {
        println!("number is divisible by 4"); // this is false
    } else if number % 3 == 0 {
        println!("number is divisible by 3"); // this is true and prints to screen
    } else if number % 2 == 0 {
        println!("number is divisible by 2"); // this is true, but doesn't print to screen because first true statement stops this if expression block
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // this shows how to use if expressions on the same line as let statements
    // note: the type of both values returned in the if/else blocks must be the same. will error otherwise
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // there are 3 kinds of loops that can be utilized in Rust
    // they are: `loop`, `while`, and `for`
    
    // this shows the use of loop
    // this will run indefinitely unless manually stopped
    // loop {
    //     println!("again!")
    // }

    // this shows the utilization of a counter and the break keyword to break out of a loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // The following shows the use of Loop Labels to disambiguate between the mutliple loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // this shows the use of a while statement
    let mut number = 3;
    
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // this shows how to easily loop through a collection with a for loop
    // note: while could also be used, but it is far more error prone compared to using this method
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the vale is: {element}")
    }

    // here is a revised version of the countdown loop from before, but using a for loop
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
