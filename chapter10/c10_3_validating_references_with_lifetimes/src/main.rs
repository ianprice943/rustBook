#![allow(unused)]
/*
// a static lifetime is one where a reference can live for the entire duration of the program
fn main() {
    let s: &'static str = "I have a static life time.";
}
*/

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    /*
    {
        let r;

        {
            let x = 5;
            r = &x; // if we were to extend this inner block, r would
                    // have no issue using the inner value of x for the
                    // length of the block
        }   // x is dropped here, but r is still trying to borrow it,
            // the borrow checker will not allow this

        println!("r: {}", r);
    }
    // the lifetime is r and the lifetime of x are not the same, so this
    // will not compile
    */


    // the following code does not have the dangling reference
    {
        let x = 5;

        let r = &x;

        println!("r: {}", r);
    }

    // generic lifetimes in functions
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);

    println!("result: {}", result);
    /*
    &i32        // a reference
    &'a i32     // a reference with an explicit lifetime
    &'a mut i32 // a mutable reference with an explicit lifetime
    */

    // passing in references with different lifetimes
    // this compiles because string1's lifetime is the entire main block,
    // and string2's lifetime is within the smaller scope. since result
    // is instantiated and used within the smaller scope where string2
    // lives, there's no issue
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // this block will not compile, as the lifetime of result is the same
    // as string2. since we attempt to use result outside of its new lifetime,
    // we get an error 
    /*
    let string3 = String::from("long string is long");
    let result2;
    {
        let string4 = String::from("xyz");
        result2 = longest(string3.as_str(), string4.as_str());
    }
    println!("The longest string is {}", result2);
    */

    // lifetime annotations in struct definitions

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // generic type parameters, trait bounds, and lifetimes together

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_with_an_announcement(
        string1.as_str(),
        string2,
        "Today is someone's birthday!",
    );
    println!("The longest string is {}", result);
}

/*
// this function does not compile because a lifetime is expected
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
*/

// we are telling the compiler that everything in this function
// has the same lifetime of 'a, including the parameters and the
// return reference will have a lifetime equal to the shortest value's
// lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}