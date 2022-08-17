// We’ve introduced three new crates: proc_macro, syn, and quote. The proc_macro 
// crate comes with Rust, so we didn’t need to add that to the dependencies in 
// Cargo.toml. The proc_macro crate is the compiler’s API that allows us to read 
// and manipulate Rust code from our code.

// The syn crate parses Rust code from a string into a data structure that we can 
// perform operations on. The quote crate turns syn data structures back into Rust
// code. These crates make it much simpler to parse any sort of Rust code we might
// want to handle: writing a full parser for Rust code is no simple task.

use proc_macro::TokenStream;
use quote::quote;
use syn;

// This function (hello_macro_derive) will be called when a user of our library
// specifies #[derive(HelloMacro)] on a type. This is possible because we have
// annotated the hello_macro_derive function here with proc_macro_derive and 
// specifyed the name HelloMacro which matches our trait name.
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    // Get an Ident (identifier) struct instance containing the name of the 
    // annotated type using ast.indent.
    let name = &ast.ident;
    // The quote! macro lets us define the Rust code we want to return. The 
    // compiler expects something different to the direct result of the quote!
    // macros execute, so we need to convert it to a TokenStream, we do this by 
    // calling the into method.
    let gen = quote! {
        // #name will be replaced with the value in the variable name
        impl HelloMacro for #name {
            fn hello_macro() {
                // The stringify! macro built into rust will convert a Rust
                // expression, such as 1+ 2 into a string literal; "1 + 2".
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}