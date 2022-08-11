fn main() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // ----- Matching named variables -----

    let a = Some(5);
    let b = 10;

    match a {
        Some(50) => println!("Got 50"),
        // will match here since match starts a new scope, and variables 
        // declared as part of a pattern inside the match expression will 
        // shadow those with the same name outside the match construct.

        // In this case we introduce a new variable b, that will match any 
        // value inside a Some value, this is a new b variable not the b we
        // declared above with the value 10. This new b binding will match
        // any value inside a Some, which is what we have in a, therefore 
        // the new b binds to the inner value of the Some a that is value 5

        // If x had been a None value instead of Some, the patterns in the
        // first two arms would not have matched, so the value would have 
        // matched the underscore.
        Some(b) => println!("Matched, y = {:?}", b),
        _ => println!("Default case, a = {:?}", a),
    }

    println!("At the end: x = {:?}, y = {}", a, b);

    // ----- Multiple Patterns -----

    let c = 1;

    match c {
        1 | 2 => println!("One or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // ----- Matching Ranges of Values -----

    let d = 5;

    match d {
        // This arm will execute when a pattern matches any of the values
        // within the range
        1..=5 => println!("One through five"),
        _ => println!("Something else"),
    }

    // Ranges are only allowed with numeric or char values

    let e = 'c';

    match e {
        'a'..='j' => println!("Early ASCII letter"),
        'k'..='z' => println!("Late ASCII letter"),
        _ => println!("Something else"),
    }

    // ----- Destructuring to break apart values -----

    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    // Create the variables f and g that match the values of the x and y 
    // fields of the p struct. This shows that the names of the variables in
    // the pattern dont have to match the field names of the struct
    let Point { x: f, y: g} = p;
    assert_eq!(0, f);
    assert_eq!(7, g);

    // This also works when the variables are of the same name as the struct
    // field(s)

    // Creates the variables x and y that match the x and y fields on the 
    // struct
    let Point {x, y} = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // Seperates point values into three cases, points that lie directly on 
    // the x axis, points that lie directly on the y axis, or neither
    match p {
        // Will match any point that lies on the x axis by specifying that 
        // the y fields matches if its value matches 0, the pattern will still
        // create an x variable that we can use in the code for this arm
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        // Will match any point on the y axis by specifying that the x field
        // matches if its value is 0 and creates a variable y
        Point { x: 0, y } => println!("On the y axis at {}", y),
        // No literals specifyed, so it matches any point and creates variables
        // for the x and y fields
        Point {x, y} => println!("On neither axis: ({}, {})", x, y),
    }

    // More destructuring, now enums
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
    }

    // More destructing, nested enums and structs
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    
    enum Message2 {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
        Message2::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }

    // Destructuring structs & tuples

    // This looks complex but it is just a simple destructuring with patterns
    let ((feet, inches), Point { x, y }) = 
        ((3, 10), Point { x: 3, y: -10 });


    // ----- Ignoring values in a pattern -----
    
    // To do this you can use the _ pattern within another pattern

    // This function will completely ignore the value passed as the first 
    // argument
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }

    foo(3, 4);

    // Ignoring parts of a value with a nested _

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        // We dont need data in the some varient
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        // In other cases if either setting_value or new_setting_value are
        // None, we want the new_setting_value to become setting_value.
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    // We can also use _ in multiple places to ignore particular values
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        // Ignore the second and fourth values of the tuple
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }

    // If you are starting a project and want to create a few variables you
    // might use but later you can prefix _ on the unused variable as follows
    let _unused = 60; // No warning about unused variable

    let s = Some(String::from("Hello!"));

    // Error, s value is moved into _s which prevents us from using s again,
    // if let Some(_s) = s {
    //     println!("found a string");
    // }

    // This works since we never bind s to anything; it isnt moved 
    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);

    // ----- Ignoring remaining parts with .. -----
    struct Vec3 {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Vec3 { x: 0, y: 0, z: 0 };

    match origin {
        // This is faster instead of having to write all the fields like, y: _,
        // z: _, etc.
        Vec3 { x, .. } => println!("x is {}", x),
    }

    // This syntax can also be used like
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        // Match the first and last value to the variables
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    // ----- Extra conditionals with match guards -----

    // A match guard is an aditional if condition specifyed after the pattern
    // in a match arm that must also match, along with the pattern matching,
    // for that arm to be chosen.

    let num = Some(4);

    match num {
        // The arm Some(4) matches to Some(x), the match guard then checks
        // whether the remainder of dividing x by 2 is equal to 0, and when it
        // is the first arm is selected.
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    let x = Some(5);
    let y = 5;

    match x {
        Some(50) => println!("Got 50"),
        // This is helpful because we dont create a new variable y that would
        // shadow the outer y, meaning we can use the outer y in the match
        // guard.
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    let x = 4;
    let y = false;

    match x {
        // This match guard applies to 4, 5 and 6
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
    
    // ----- @ Bindings -----

    // The @ operator lets us create a variable that holds a value at the same
    // time we're testing that value to see whether it matches a pattern.
    enum Message3 {
        Hello { id: i32 },
    }

    let msg = Message3::Hello { id: 5 };

    match msg {
        // We want to test that the Message3::Hello id field is within the range
        // 3..=7, but we also want to bind the value to the variable id_variable
        // so we can use it in the code associated with the arm.
        Message3::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        // In this arm we only have a range specified in the pattern, the code
        // associated with the arm does not have a variable that contains the 
        // actual value of the id field.
        Message3::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        // We have the value available to use in the arms code in a variable named
        // id. We have used the struct field shorthand syntax.
        Message3::Hello { id } => println!("Found some other id: {}", id),
    }

    // Using @ lets you test a value and save it in a variable within one pattern
}
