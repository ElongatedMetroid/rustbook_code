// Defines a trait named Summary
// Inside the curly braces we declare method signatures that 
// describe the behaviors of the types that implement this trait
pub trait Summary {
    fn summarize(&self) -> String;
    // sometimes its useful to have a default behaviour for some methods
    fn bird_noise(&self) -> String {
        String::from("Tweet Tweet, Caw Caw")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
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
}