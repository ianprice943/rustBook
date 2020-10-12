use std::fs::remove_file;
use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    // another way to write this without match statements would be to do
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    // this uses closures, which will be discussed in chapter 13

    // remove file to guarentee a panic in next code example
    remove_file("hello.txt");

    // shortcuts for panic on error: unwrap and expect
    //let f = File::open("hello.txt").unwrap();   // unwrap() will return Ok if no errors, and panic! if not
    let f = File::open("hello.txt").expect("Failed to open hello.txt"); // expect() lets us customize the error message 
                                                                        // to convey more clarity

    // the ? operator can be used in functions that return result
    //let f = File::open("hello.txt")?;   // this compile errors because main() returns () not Result<T, E>
                                        // however, you can change main()'s return type to Result<T, E>
    /*
    fun main() -> Result<(), Box<dyn error>> {
        let f = File::open("hello.txt")?;
        Ok(())
    }
    */
    // this is a valid main function in Rust
}

// propagating errors
fn ready_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// a shortcut for propagating errors: the ? operator
fn ready_username_from_file_shortcut() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// chaining with the ? operator

fn ready_username_from_file_chaining() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

