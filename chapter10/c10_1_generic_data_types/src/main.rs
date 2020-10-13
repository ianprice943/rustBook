// generics in structs
struct Point<T, U> {
    x: T,
    y: U,
}
impl<T, U> Point<T, U> {    // this method takes the point and another point, and returns a new point with the 
                            // x value of the calling point, and the y value of the passed point
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// generics in enums
enum Option<T> {
    Some(T),
    None,
}
enum Result <T, E> {
    Ok(T),
    Err(E),
}

// generics in method definitions
struct Point2<T> {
    x: T,
    y: T,
}
impl<T> Point2<T> { // need to specify that we're writing a method for type Point2<T>
    fn x(&self) -> &T {
        &self.x
    }
}
impl Point2<f32> { // here we specify that we are implementing specifically an f32 set of methods
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, &100);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
    assert_eq!(result, &'y');

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    //let wont_work = Point { x: 5, y: 4.0 };   // our struct only allowed one generic type previously
    let integer_and_float = Point { x: 5, y: 4.0 };

    let p = Point2 { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// the two above work, but they have the same inner code. We can replace these two functions
// with a single generic function
/*
fn largest<T>(list: &[T]) -> &T {
    let mut largest = list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
*/