fn main() {
    let s = String::from("hello");  //string literal stored on the heap instead of the stack
                //^^ double dolon allows the use of "from" from the String namespace
    
    let mut str = String::from("hello");  //this kind of string CAN be mutated
    str.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", str);
}
