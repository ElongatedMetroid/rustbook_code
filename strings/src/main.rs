use std::fmt::format;

fn main() {
    // create a new [empty] string
    let s = String::new();

    let data = "initial contents";

    // convert &str to a String
    let s = data.to_string();

    println!("{}", s);

    // Strings are UTF-8 encoded so they can hold any encoded data into them
    let box_drawings = String::from("╯ ╰ ╳ ┇");

    println!("{}", box_drawings);

    // push more strings to strings with the push_str() method
    let mut s = String::from("Hello");
    s.push_str(", world!");

    println!("{}", s);

    // you can push single chars to strings with the push() method
    let mut s = String::from("Ratio + ");
    s.push('L');

    println!("{}", s);

    let s1 = String::from("Hello");
    let s2 = String::from(", world!");

    // s1 had been moved and can no longer be used
    // add a reference of the second string to the first string
    let s3 = s1 + &s2;

    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // s will now contain tic-tac-toe but this isnt the most readable way
    let s = s1 + "-" + &s2 + "-" + &s3;

    println!("{}", s);

    // instead of doing that you can use the format macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    
    // easier to read and does not take ownership of any of its parameters
    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("{}", s);

    // with Rust Strings you cannot acess them by indexing
    // let i = s1[0]; <-- invalid syntax
    // this is because some UTF-8 characters are more than 1 byte long

    let s = String::from("नमस्ते");

    // to iterate over charaters of a string you can iterate over the result of the char method
    for c in s.chars() {
        println!("{}", c);
    }
}
