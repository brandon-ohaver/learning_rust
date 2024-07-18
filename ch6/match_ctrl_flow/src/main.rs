#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Missouri,
    Illinois,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin { // this allows us to unwrap the Coin enum from the value
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => { // this allows us to unwrap the state enum from the value
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Illinois));
}
