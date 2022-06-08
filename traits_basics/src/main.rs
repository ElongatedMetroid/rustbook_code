use std::fmt::{Debug, Display};
use traits_basics::{Summary, Tweet};

// in trait bounds we specify that the item must be of a type that implements the summary trait
// this function can now be called by either passing in NewsArticle or Tweet, code that uses a different types wont compile (Since they dont implement Summary)
pub fn notify<T: Summary>(item: T) {
    println!("Breaking News! {}", item.summarize());
}

// T can be any type that implements Summary and display, U can be any type that implements Clone and Debug
// This is valid but it is difficult to read
// fn foo<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32
fn foo<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    0
}

struct Pair<T> {
    x: T,
    y: T,
}

// always implemented
impl<T> Pair<T>{
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// the method cmp_display is only implemented if the type of the Pair implements the Display and PartialOrd traits
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x: {}", self.x);
        } else {
            println!("The largest number is y: {}", self.y);
        }
    }
}

// implment ToString on any type that implements the Display trait
// impl<T: Display> ToString for T { }

fn main() {
    let tweet = Tweet {
        username: String::from("Demarkus Shawn"),
        content: String::from("as you already are familiar with obamas apple pie..."),
        reply: false,
        retweet: false,
    };

    println!("new tweet: {}", tweet.summarize());
    println!("New bird: {}", tweet.bird_noise());

    notify(tweet);
}
