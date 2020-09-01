#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),                        // a lot of boiler plate just to check for 3
    }

    if let Some(3) = some_u8_value {    // behaves the same way as above, but with less code.
        println!("three");              // this does however remove the exhaustive checking of
    }                                   // match.

    // these two give the same result, it's up to the programmer to decide which is more appropriate
    // for their use case
    let coin = Coin::Penny;
    let mut count = 0;
    match &coin {   // & added to allow borrowing instead of move
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
