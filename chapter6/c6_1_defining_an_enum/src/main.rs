/*
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}*/

// combine struct and enum into a more concise enum
enum IpAddr {
    V4(u8, u8, u8, u8), // or V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
        fn call(&self) {
            // method body would be defined here
            println!("call() was called");
            
        }
    }


fn main() {
    /*
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };
    */

    let home = IpAddr::V4(127, 0, 0, 1); // or IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    //the Option enum

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    //let sum = x + y;  //this is not allowed because rust doesn't know how to
                        //add an i8 and an Option<i8>
    
  
}

//fn route(ip_kind: IpAddrKind) {}
