fn main() {
    // if Expressions
    /*
    //let number = 3;
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    */

    //non bool example
    /*
    let number = 3;

    if number {
        println!("number was three");
    }

    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
    */

    //handling multiple conditions with else if
    /*
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    */

    //using if in a let statement
    /*
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
    */

    //incompatible types
    /*
    let condition = true;
    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {}", number);
    */

    //repeating code with loop
    /*
    loop {
        println!("again");
    }
    */

    //returning values from loops
    /*
    let mut counter = 6;
    
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }   
    };

    println!("The result is {}", result);
    */

    //conditional loops with while
    /*
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
    */

    //looping through a collection with for
    /*
    //this works, but is slow due to doing conditional checks on each iteration of the while loop
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
    */
    /*
    //this is more concise, and is less error prone should the length of the array change
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
    */

    //an example of both range and the reverse method of ranges
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
