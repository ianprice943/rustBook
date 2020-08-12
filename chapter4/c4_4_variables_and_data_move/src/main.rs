fn main() {
    let x = 5;
    let y = x; //Two variables representing 5 have been pushed on the stack.
               //Stack is used because integer types have a fixed memory size.
    
    let s1 = String::from("hello");
    let s2 = s1; //pointer for s1 and s2 point at the same location in the heap
                 //s1 is also considered invalid now to avoid "double free" memory corruption
}
