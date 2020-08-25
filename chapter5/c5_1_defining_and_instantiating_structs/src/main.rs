// Listing 5-2

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Tuple Structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // one can get a value from the struct by using dot notation such as
    // user1.email
    user1.email = String::from("anotheremail@example.com"); // this only works if we declare email as mutable
    // however, an entire struct must be mutable in order to change the value of any piece of data within
    // hence why user1 is declared as mutable

    // we can create new instances of a struct from another like so
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    // but we can also use this update syntax to short hand what we're pulling in from the older instance
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    // creating instances of Tuple Structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

/*
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
*/
// short hand for when variables and fields have the same name
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}