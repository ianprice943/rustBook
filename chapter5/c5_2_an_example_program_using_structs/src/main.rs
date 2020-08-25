/*
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {   // while taking two parameters and getting
    width * height                          // one output isn't necessarily bad, in
}                                           // this case the parameters are related and 
*/                                          // that isn't clear

// refactoring with tuples
/*
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {    // in a way, this is better and has more structure
    dimensions.0 * dimensions.1             // however this can be unclear because tuples don't
}                                           // name their elements
*/

// refactoring with structs

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    //println!("rect1 is {}", rect1); // printing out our struct like this will not work, because we have no way to format it yet
    println!("rect1 is {:?}", rect1);   // this also errors until we add #[derive(Debug)] before our struct declaration
    println!("rect1 is {:#?}", rect1);   // this prints the struct in a prettier and more human readable way
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
