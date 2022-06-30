
/// Implementation of a Con list that can only hold i32 values.
/// 
/// A Con list is a data structure that holds the value of the current element, and the next item
/// when you reach the end of the list it will contain the Nil value.
enum List {
    // indirectly store the list so rust knows the size,
    // storing indirectly means that we will change the data structure
    // to store the value indirectly by storing a pointer to the value 
    // instead
    // Because Box<T> is a pointer rust will always know how much space
    // a Box<T> needs
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    // The variable b is a pointer to the value 5,
    // the value 5 is allocated on the heap
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
}
