use std::fmt::Display;

fn main() {
    // this will end in an error
    // { // outer scope
    //     let r;

    //     { // inner scope
    //         let x = 5;
               // set r to a reference to x
    //         r = &x;
    //     } // x goes out of scope (it has been freed and no longer exists)

    //     println!("r: {}", r);
    // }

    // This works since x is in scope the entire time
    {
        let x = 5;
        let r = &x;
        println!("r: {}", r);
    }

    let string1 = String::from("Hello");
    let string2 = "Bye";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // Lifetime annotation syntax
    // &i32         // a reference
    // &'a i32      // a reference with an explicit lifetime
    // &'a mut i32  // a mutable reference with an explicit lifetime

    // generic lifetime parameter defined in angle brackets
    // This annotation means an instance of ImportantExcerpt cannot outlive the reference it holds in its part field
    struct ImportantExcerpt<'a> {
        // holds one field which is a string slice
        part: &'a str,
    }

    let novel = String::from("Call me Ishmeal. Some years ago...");
    let first_sentance = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    // creates an instance of the ImportantExcerpt struct that holds the first sentance of the String owned by novel
    let i = ImportantExcerpt { part: first_sentance };

    // static lifetimes
    // All string literals are stored in the binary of the program and are always availible, you can specify static with the special 'static lifetime
    let s: &'static str = "I have a static lifetime";


}

// This function will cause a compilation error saying we need to provide generic lifetime parameters
// This is because we dont know if the if statment will return a reference to x or y
// we also do not know the concrete lifetimes of the references passed inred
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// First we declare the generic lifetime parameter in angle brackets
// we then specify that all references in the parameters and return value will have the same lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// in this case we have made a lifetime parameter 'a for the parameter x and the return type,
// but not for y, because the lifetime of y does not have any relationship with the lifetime
// of x or the return value
fn foo<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// this function implements Generic Type Parameters, Trait Bounds, and lifetimes
fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    // T needs display because it will be printed
    where T: Display
{
    println!("Accouncement {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}