fn main() {
    // ----- Function Pointers -----

    fn add_one(x: i32) -> i32 {
        x + 1
    }
    
    // Take a function pointer and i32 as arguments
    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    let answer = do_twice(add_one, 5);

    println!("answer is: {}", answer);

    // fn is a type rather than a trait, so we specify fn as the
    // parameter type directly rather than declaring a generic type
    // parameter with one of the Fn traits as a trait bound.

    // Function pointers implement all three of the closure traits
    // (Fn, FnMut, and FnOnce), meaning you can always pass a 
    // function pointer as an argument for a function that expects
    // a closure. Its best to write functions using a generic type
    // and one of the closure traits so your functions can accept
    // either functions or closures.

    // One example where you would only accept fn and not closures
    // is when interfacing with external code that does not have
    // closures.

    // An example of where you could use either a closure defined
    // inline or a named function is the map method provided by the
    // Iterator trait in the stdlib

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = 
        // Get an iterator to the list of numbers, create a closure
        // and take in the current element as the parameter, call
        // the to_string method on each of those elements.
        list_of_numbers.iter().map(|i| i.to_string()).collect();

    println!("{:?}", list_of_strings);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = 
        // Get an iterator to the list of numbers, call the 
        // to_string method on each of the elements (call the 
        // to_string method with the current element supplied as the
        // argument),
        list_of_numbers.iter().map(ToString::to_string).collect();

    println!("{:?}", list_of_strings);

    // Each enum variant that we define also becomes and initializer
    // function, these initializer functions can be used as function
    // pointers that implement the closure traits.
    #[derive(Debug)]
    enum Status {
        Value(u32),
        _Stop,
    }

    // Create a vector of Status::Value instance (by using the enum
    // initializer functions)
    let list_of_statuses: Vec<Status> = 
        // Create each instance using each u32 value in the range
        // map is called on using the initializer function
        (0u32..20).map(Status::Value).collect();

    println!("{:?}", list_of_statuses);

    // Using closures
    let list_of_statuses: Vec<Status> = 
        (0u32..20).map(|i| Status::Value(i)).collect();

    println!("{:?}", list_of_statuses);

    // ----- Returning Closures -----

    // Closures are represented by traits, which means you cant
    // return closures directly, in most cases where you might want
    // to return a trait you can instead use the concrete type that
    // implements the trait as the return value of the function.
    // However that is not possible with closures because they dont
    // have a concrete type that is returnable

    // Fails, Rust does not know how much space it will need to 
    // store the closure
    // fn returns_closure() -> dyn Fn(i32) -> i32 {
    //     |x| x + 1
    // }

    // Instead we can use a trait object
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
}
