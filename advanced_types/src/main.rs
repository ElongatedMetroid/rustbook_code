fn main() {
    // ----- Creating Type Synonyms with Type Aliases -----

    // Create a type alias for i32
    type Kilometers = i32;

    // Kilometers is a synonym for i32, Kilometers is NOT a seperate
    // new type, values that have the type kilometers will be 
    // treated the same as values of type i32.
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    // The main usecase for type synonyms is to reduce repetition
    // Lets say we have a long type and we dont want to type it 
    // every time we use it.
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let _f: Thunk = Box::new(|| println!("hi"));

    fn _takes_long_type(_f: Thunk) {
        todo!()
    }

    fn _returns_long_type() -> Thunk {
        todo!()
    }
    // In turn the code turns out much cleaner and easier to read
    // and write.

    // Types aliases are commonly used with the Result<T, E> type
    // for reducing repetition. Condsider the std::io module in the
    // stdlib I/O operations often return a Result<T, E> to handle
    // situations when operations fail to work. This library has a
    // std::io::Error struct that represents all possible I/O errors
    use std::fmt;
    use std::io::Error;

    // Result<..., Error> is repeated a lot, as such, std::io has
    // this type alias declaration
    type Result<T> = std::result::Result<T, std::io::Error>;

    pub trait Write {
        fn write(&mut self, buf: &[u8]) 
            -> Result<usize>;
        fn flush(&mut self) 
            -> Result<()>;

        fn write_all(&mut self, buf: &[u8]) 
            -> Result<()>;
        fn write_fmt(&mut self, fmt: fmt::Arguments) 
            -> Result<()>;
    }

    // ----- The Never Type that Never Returns -----

    // Rust has a special type named ! thats know in type theory
    // as the empty sype because it has no values. We prefer to call
    // it the never type because it stands in the place of the 
    // return type when a function will never return 

    // The function bar returns never
    fn _bar() -> ! {
        panic!();
    }
    
    loop {
        let guess = String::from("50");
        
        // All match arms must return the same type so how is one 
        // returning num and the other returning what continue returns
        let _guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Fails
        // let _guess: u32 = match guess.trim().parse() {
        //     Ok(num) => num,
        //     Err(_) => continue,
        // };

        break;
    }

    // Continue returns a ! value, because when Rust computes the 
    // type of guess, it looks at both match arms, the former with
    // a value of u32, and the latter with a ! value, since ! can
    // never have a value, Rust decides that the type of guess is 
    // u32.

    // The never type is also useful with the panic! macro, panic
    // has the type of ! so you could use a panic! macro in match.

    // ----- Dynamically Sized Types and the Sized Trait -----

    // Rust needs to know certain detail about its types, such as
    // how much space to allocate for a value of a particular type
    // This leaves one corner a bit confusing at first, the concept
    // of dynamically sized types.
    
    // Lets look into the details of a dynamically sized type called
    // str (not &str). We cant know how long the string is until
    // runtime, meaning we cant create a variable of type str or 
    // can we take an argument of type str.

    // The following will not work since all the values of a type 
    // must use the same amount of memory.
    // let s1: str = "Hello there!";
    // let s2: str = "How's it going?";

    // Instead of doing the above we can make &str's, an &str is 
    // two values, the address of the str and its length, as such
    // we can know the size of a &str at compile time

    // To work with DSTs, Rust provides the Sized trait to determine
    // whether or not a type’s size is known at compile time. This 
    // trait is automatically implemented for everything whose size 
    // is known at compile time. In addition, Rust implicitly adds a
    // bound on Sized to every generic function. That is, a generic
    // function definition like this:

    // fn generic<T>(t: T) {
    //     // --snip--
    // }

    // -->

    fn _generic<T: Sized>(_t: T) {
        // --snip--
    }

    // By default generic functions will work only on types that 
    // have a known size at compile time, however use the following
    // special syntax to relax this restriction.

    fn __generic<T: ?Sized>(_t: &T) {
        // --snip--
    }

    // A trait bound of ?Sized means T may or may not be Sized, and
    // this notation overrides the default that generic types must
    // have a known size at compile time.
}
