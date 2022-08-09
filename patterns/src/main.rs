fn main() {
    // ----- Match arms -----
    let value = 6;
    match value {
        // Pattern   // Expression
        3 => println!("Expression"),
        _ => println!("No matches"),
    }

    // ----- If statements -----
    // (if let, else if, else if let, else)

    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    // Extract the value inside favorite color if it is a Some, if it is
    // None (since this is an Option enum) dont execute the expression
    // inside the if let
    if let Some(color) = favorite_color {
        println!(
            "Using your favorite color {}, as the background",
            color
        );
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } 
    // This line will introduce a new shadowed age varable that contains
    // the value inside the Ok variant, this means we need to place
    // the if age > 30 condition within that block, since we cant combine
    // these two conditions into if let Ok(age) = age && age > 30.
    else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the backgroudn color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // ----- While let conditional loops -----

    // Similar to if let, while let conditional loops allow a while loop
    // to run for as long as a pattern continues to match.

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    // Keep setting Some(top) to the last element from the vector until
    // we reach the end/the pop method returns None.
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // ----- for loops -----

    // for x in y, x is the pattern

    let v = vec!['a', 'b', 'c'];

    // the index and value are the patterns
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // ----- let statements -----

    // Let satements are actually patterns, the following example shows
    // let pattern = expression;

    // Match the tuple against a pattern, rust compares the value
    // (1, 2, 3) to the pattern (x, y, z) and sees that the value matches
    // the pattern, so rust binds 1 to x, 2 to y, and 3 to z.
    let (x, y, z) = (1, 2, 3);
    println!("{} {} {}", x, y, z);

    // ----- function parameters -----

    // x is a pattern
    // fn foo(x: i32) {}

    let point = (3, 5);
    // The values &(3, 5) match the pattern &(x, y)
    print_coordinates(&point);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
