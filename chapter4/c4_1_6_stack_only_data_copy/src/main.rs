fn main() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y); //this is valid because integers are easy to copy on
                                      //the stack and have a known memory size at compile time
}
