fn main() {
    // The term macro refers to a family of features in Rust,
    // declarative macros with macro_rules!
    // and three kinds of procedural macros:
    // Custom #[derive] macros that specify code added with the derive 
    // attribute used on structs and enums,
    // Attribute-like macros that define custome attributes usable on any 
    // item, and
    // Function-like macros that look like function calls but operate on the
    // tokens specifed as their argument

    // ----- The Difference Between Macros and Functions -----

    // Essentially, macros are a way of writing code that writes other code
    // which is known as metaprogramming. Take the derive attribute, which
    // generates an implementation of various traits for you. We have also 
    // used teh println! and vec! macros. All these macros expand to produce
    // more code than the code you have written manually.

    // Metaprogramming is useful for reducing the amount of code you have to
    // write and maintain, which is also one of the roles of functions, 
    // however, macros have some additional powers that functions dont.

    // A function signature must declare the number and type of parameters 
    // the function has. But macros can take a variable number of parameters.
    // Macros are also expanded before the compiler interprets the meaning of
    // the code, so a macro can implement a trait on a given type. A function
    // cant because it gets called at runtime and a trait needs to be 
    // implemented at compile time.

    // A downside of implementing a macro instead of a function is that macro
    // definitions are more complex than function definitions since you are
    // writing Rust code that writes Rust code. Due to this macro definitions
    // are generally more difficult to read, understand, and maintain than
    // function definitions.

    // Macros must also be brought into scope before you call them in a file,
    // as opposed to functions that you can define anywhere and call anywhere   
  
    // ----- Declarative Macros with marco_rules! for General -----
    // ----- Metaprogramming                                  -----

    // Declarative macros allow you to writ esomething similar to a match
    // expression. (As you already know) Match expressions are control
    // structures that take an expression, compare the resulting value of the
    // expression to patterns, then run the code associated with the matching
    // pattern.

    // Macros also compare a value to patterns that are associated with 
    // particular code: in this situation the value is the literal Rust source
    // code passed to the macro; the patterns are compared with the structure
    // of that source code; and the code associated with each pattern, when 
    // matched, replaces the code passed to the macro. This happens during
    // compilation.

    // To define a macro use the marco_rules! construct.

    // Lets take a look at how the vec! macro is defined

    // The vec! macro is used to make a new vector, but it can be used to make
    // a vector of 3 integers, 7 strings, 1000 string slices, etc., but with
    // a function you cannot do this since we dont know the number or type of
    // values up front.
    let _v: Vec<u32> = vec![1, 2, 3];

    // Heres a simplified version of the vec! macro

    // This annotation indicates that this macro should be made available
    // whenever the crate in which the macro is defined is brought into scope.
    // Without this annoctation, the macro cannot be brought into scope
    #[macro_export]
    // The macro definition is started with macro_rules! and the name of the
    // macro we are defining (without the ! symbol). The name is then followed
    // by curly brackets denoting the body of the macro definition
    macro_rules! simple_vec {
        // The structure in the body of this macro is similar to the structure
        // of a match expression, here there is one arm with the pattern
        // `( $( $x:expr ),* )` followed by a block of code associated with
        // this pattern.. If the pattern matches, the associated block of 
        // code will be emitted.

        // Valid pattern syntax in macro definitions is different than the
        // other pattern syntax, because macro patterns are matched agaist
        // Rust code structure rather than values.

        // First we use a set of parentheses to encompass the whole pattern.
        // The $ sign is then used to declare a variable in the macro system
        // that will contain the Rust code matching the pattern. The $ sign
        // makes it clear this is a macro variable as opposed to a regular
        // Rust variable. Next comes a set of parentheses that capture values
        // that match the pattern within the parentheses for use in the 
        // replacement code. Within the $() is $x:expr, which matches any
        // Rust expression and gives the expression the name $x.

        // The comma following the $() indicates that a literal comma 
        // separator character could optionally appear after the code that 
        // matches the code in $(). The * specifies that the pattern matches
        // zero or more of whatever precedes the *

        // When we call this macro with simple_vec![1, 2, 3]; the $x pattern
        // matches three times with the three expressions 1, 2, and 3.
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                // temp_vec.push() within $()* is generated for each part that
                // matches $() in the pattern zero or more times depending on
                // how many times the pattern matches.
                $(
                    // The $x is replaced with each expression matched
                    temp_vec.push($x);
                )*
                temp_vec
            }

            // In the end, the code in this arm generated will be the
            // following

            // {
            //     let mut temp_vec = Vec::new();
            //     temp_vec.push(1);
            //     temp_vec.push(2);
            //     temp_vec.push(3);
            //     temp_vec
            // }
        };
    }

    let v: Vec<&str> = simple_vec!["Hello", "world"];

    println!("{:?}", v);

    // ----- Procedural Macros for Generating Code from Attributes -----

    // Another form of macros is the procedural macro, which acts more like a 
    // function (and is a type of procedure). Procedural macros do accept some
    // code as input, operate on that code, and produce some code as an output
    // rather than matching against patterns and replacing the code with other
    // code as declarative macros do. The three kinds of procedural macros are
    // custom derive, attribute-like, function-like, and all work in a similar
    // fashion.

    // When creating procedural macros, the definitions must reside in their own
    // crate with a special crate type.

    // Below shows how to define a procedural macro, where some_attribute is a 
    // placeholder for using a specific macro variety.

    // use proc_macro;

    // #[some_attribute]
    // pub fn some_name(input: TokenStream) -> TokenStream {
    // }

    // The function that defines a procedural macro takes a TokenSteam as an
    // input and produces a TokenStream as an output. The TokenStream type is 
    // defined by the proc_macro crate that is included with Rust and represents
    // a sequence of tokens. This is the core of the macro: the source code that
    // the macro is operating on makes up the input TokenStream, and the code the
    // macro produces is the output TokenStream. The function also has an 
    // attribute attached to it that specifies which kind of procedural macro we
    // are creating. We can have multiple kinds of procedural macros in the same
    // crate.

    // ----- How to Write a Custom derive Macro -----

    // Take a look at the hello_macro crate inside the folder the current crate
    // is contained in.

    use hello_macro::HelloMacro;
    use hello_macro_derive::HelloMacro;

    // Use our HelloMacro custom derive macro to implement the HelloMacro trait
    #[derive(HelloMacro)]
    struct Pancakes;

    // Call the implemented method
    Pancakes::hello_macro();

    // ----- Attribute-like macros -----

    // Attribute-like macros are similar to custom derive macros but instead of
    // generating code for the derive attribute, they allow you to create new 
    // attributes they are also more flexible, derive only works for structs and
    // enums; attributes can be applied to other items aswell such as functions.

    // say you have an attribute named route that annotates functions when using 
    // a web application framework:

    // Defines the #[route] attribute
    // Here there is two parameters of type TokenStream. The first is for the
    // contents of the attribute: the `GET, "/"` part. The second is the body
    // of the item the attribute is attached to: in this case, fn index() {}
    // Other than that attribute-like macros work the same way as custom derive
    // macros: you create a crate with the proc-macro crate type and implement
    // a function that generates the code you want.

    // #[proc_macro_attribute]
    // pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}

    // #[route(GET, "/")]
    // fn index() {}

    // ----- Function-like macros -----

    // Function-like macros define macros that look like function calls. Similarly
    // to macro_rules! macros, they're more flexible than functions; for example
    // they can take an unknown number of arguments. However, macro_rules! macros
    // can be defined only using the match-like syntax we saw earlier. Function-
    // like macros take a TokenStream parameter and their definition manipulates
    // that TokenStream using Rust code as the other two types of procedural 
    // macros do. An example of a function-like macro is an sql! macro that might
    // be called like so:

    // let sql = sql!(SELECT * FROM posts WHERE id=1);

    // This macro would parse the SQL statment inside it and check that its 
    // syntactically correct, which is much more complex processing than a 
    // macro_rules! macro can do.

    // The sql! macro would be defined like this:

    // #[proc_macro]
    // pub fn sql(input: TokenStream) -> TokenStream {}

    // This definition is similar to the custom derive macros signature, we 
    // receive the tokens that are inside the parentheses and return the code we
    // wanted to generate.

}
