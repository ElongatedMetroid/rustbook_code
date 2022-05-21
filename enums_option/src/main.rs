// The Option enum defined in the standard library can either be Something or Nothing

// enum Option<T>{
//     Some(T),
//     None,
// }
// Rust allows you to use Some and None directly without the Option:: prefix
// Think of T as allowing the enum to hold one piece of data of any type

fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    // None is technicaly the same thing as null
    let absent_number: Option<i32> = None;

    let five = Some(5);

    let six = plus_one(five);
    let none = plus_one(None);

    // if you just want to do something with one match (ex Some(3)) you can use if let
    // takes pattern and expression seperated by '='
    let some_u8_value = Some(3u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
    // compared to
    // match some_u8_value {
    //     Some(3) => println!("Three");
    //     _ => (),
    // }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// if we only want 
