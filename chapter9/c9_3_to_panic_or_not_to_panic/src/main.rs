fn main() {
    use std::net::IpAddr;

    // since this is a hard coded string it's reasonable to assume it cannot error out.
    // if this was a user passed string however, you should do proper error handling
    let home: IpAddr = "127.0.0.1".parse().unwrap();

    // creating custom types for validation

    // snippet from the guessing game with an update to see if the guess is less than 1
    // or greater than 100
    /*
    loop {
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        match guess.cmp(&secret_number) {

        }
    }
    */

    // instead of checking that the guess is valid in many places throughout our code, it
    // would be useful to create a custom type that does the validation for us

    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess must be between 1 and 100, got {}.", value);
            }

            Guess { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }

    let guess = Guess::new(45);
    println!("The guess was {}.", guess.value);
}
