fn main() {
    // declared type vector
    let v: Vec<i32> = Vec::new();

    // inferred type vector
    let v = vec![1,2,3];    // we didn't give 'v' a type, but rust made it type
                            // 'i32' as that's the default number type

    // updating a vector by pushing elements on to it
    let mut v = Vec::new(); // again, we don't declare a type, because we start
                            // pushing numbers on to 'v' and it infers the type
                            // to be 'i32'

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // dropping a vector drops its elements

    {
        let v = vec![1,2,3,4];
        // do stuff with 'v'
    }   // 'v' goes out of scope here, and its memory is freed

    // reading elements of vectors
    // there are two ways to read an element, with brackets [], or with get()
    let v = vec![1,2,3,4,5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // how do these two ways to read handle out of bounds accesses?
    let v = vec![1,2,3,4,5];

    let does_not_exist = &v[100];       // this version will cause the program to panic and crash
    let does_not_exist = v.get(100);    // this version will return 'None' without panicking

    // when the program has a valid reference, the borrow checker enforces ownership rules
    // the following code will not compile because it has both a mutable and immutable reference
    /*
    let mut v = vec![1,2,3,4,5];
    let first = &v[0];
    v.push(6);
    println!("The first element is: {}", first);
    */

    // iterating over the values in a vector
    let v = vec![100,32,57];
    for i in &v {
        println!("{}", i);
    }

    // we can also iterate over mutable refernces to each element in a mutable vector in order to
    // change all the elements
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // sometimes storing lists of different types is desired, this is where enums step in to help
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    // rust needs to know the types so that it can allocate the right amount of memory ahead of time.
    // pairing enums, vectors, and match expressions also allows all cases of data to be properly handled.
}