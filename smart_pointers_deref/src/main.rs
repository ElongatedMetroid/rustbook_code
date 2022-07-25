use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    // defines an associated type for the Deref
    // a slightly different way of declaring a generic parameter
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // return a reference to the value we want to acess with the dereference operator
        // this is because rust will substitute the * operator with a call to this method,
        // *y = *(y.deref()), the reason deref returns a reference is because rust will put
        // a plain reference outside of the paraenthesis (this is because of the ownership
        // system)
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // to dereference y we need to implement the Deref trait for MyBox
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Nate"));
    hello(&m);
}

fn hello(name: &str) {
    println!("Hello {}", name);
}
