// The example bellow with a box implementation will fail,
// since the Cons own the data they hold, so when the b
// list is created a is moved into b and then b owns a
// we then cannot create c since a has already been moved

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// use List::{Cons, Nil};

// fn main() {
//     // List that contains 5 and 10
//     let a = Cons(5, 
//         Box::new(Cons(10,
//             Box::new(Nil))));

//     let b = Cons(3, Box::new(a));
//     let c = Cons(4, Box::new(a));
// }

// To make this work we can use an Rc instead of a Box

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    // Create a new list with elements 5 and 10
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("Count after creating b = {}", Rc::strong_count(&a));
    
    {
        let c = Cons(4, Rc::clone(&a));
        println!("Count after creating c = {}", Rc::strong_count(&a));
    }

    println!("Count after c goes out of scope = {}", Rc::strong_count(&a));
}