#![allow(unused_variables)]
fn main() {
    //numbers
    let x = 2.0;

    let y: f32 = 3.0;

    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;

    let remainder = 43 % 5;

    //booleans
    let t = true;

    let f: bool = false;

    //characters
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';

    //compound types (tuples)
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    //or
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
    //this allows for destructuring
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    
    let six_point_four = x.1;

    let one = x.2;

    //arrays
    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    //declaring type and size of array
    let a: [i32; 5] =  [1, 2, 3, 4, 5];
    //declaring initial values of array
    let a = [3; 5]; //this is equivalent to let a = [3, 3, 3, 3, 3];

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    let index = 10;

    let element = a[index]; //this will cause an array index out of bound runtime error
    
    println!("The value of element is: {}", element);
}
