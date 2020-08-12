fn main() {
    let s = String::from("hello");  // s comese into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    //println!("I'm trying to use s: {}", s);   // leaving this line uncommented
                                                // causes a compiler error, as 's'
                                                // was moved into 'takes_ownership()'
    
    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function 
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

    println!("I'm using x again: {}", x);   // x is safe to use again as it's type is Copy
}

fn takes_ownership(some_string: String) {   // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and 'drop' is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {  // some_intege comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.