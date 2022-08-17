// This crate will define a trait named HelloMacro with one associated function
// named hello_macro. Rather than making the users implement the HelloMacro trait
// for each of their type, we'll provide a procedural macro so users can annotate
// their type with #[derive(HelloMacro)] to get a default implementation of the
// hello_macro function.

pub trait HelloMacro {
    // We will create a macro to create a default implementation of this type,
    // the macro will have this type print "Hello, Macro! My name is [TypeName]"
    // Where TypeName will be the name of the type the trait is implemented on.
    // This is not possible with a default implementation.
    fn hello_macro();
}

// Procedural macros need to be in there own crate, ending in _derive, for a crate
// named foo, a custom derive procedural macro crate is called foo_derive.