fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s //at this point s has been freed, but we're trying to return a reference to it
    //the solution would be to just return s itself like below
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}