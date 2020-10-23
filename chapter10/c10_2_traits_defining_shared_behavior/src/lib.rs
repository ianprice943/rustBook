pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    /*fn summarize(&self) -> String {   // leaving this commented out will force default behavior
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }*/
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// traits as parameters

// this function doesn't ask for a type on the item, it instead asks for any parameter which
// implements the Summary trait
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// trait bound syntax

// impl Trait works for straightforward cases, but is syntactic sugar for a longer form known as a "trait bound"

pub fn notify_tbound<T: Summary>(item: &T) {
    println!("Breaking news!{}", item.summarize());
}

// this works if we don't care what the types of item1 and item2 are
pub fn notify_extra_params(item1: &impl Summary, item2: &impl Summary) {
    println!("Double breaking news!{}, {}", item1.summarize(), item2.summarize());
}

// if we do want to ensure they're both the same type, we can do this:
pub fn notify_extra_params_tbound<T: Summary>(item1: &T, item2: &T) {
    println!("Double breaking news, this time with same types!{}, {}", item1.summarize(), item2.summarize());
}

// specifying multiple trait bounds with the + syntax

// Display isn't defined in this scope, so this will no longer compile
pub fn notify_plus(item: &(impl Summary + Display)) {}

// also valid for trait bounds on generic types
pub fn notify_plus_tbound<T: Summary + Display>(item: &T) {}

// clearer trait bounds with where clauses

// instead of writing this
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

// we can do 
fn some_function_where<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{}

// returning types that implement traits

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// this code will not compile, due to restrictions on how impl Trait is implemented
// at the compiler level
fn returns_summarizable_switch(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}

// use trait bonds to conditionally implement methods

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

// all versions of Pair will be able to call new()
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// however, only Pairs which implement Display and PartialOrd can call cmp_display()
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}