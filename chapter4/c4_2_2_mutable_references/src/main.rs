fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    /*
    let r1 = &mut s;    //this will not compile because we can only have
    let r2 = &mut s;    //one mutable rference to a data point at a time

    {
        let r1 = &mut s;
    } // r1 leaves scope here, so r2 would be valid after this point

    let r2 = &mut s;

    println!("{}, {}", r1, r2);
    */


    //combining mutable and immutable references can work, but not always
    /* 
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
    */

    //this combination works becausee the last immutable reference is used before
    //the mutable reference is made
    /*
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
    */
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}