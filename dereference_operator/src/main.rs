fn main() {
    // holds the value 5
    let x = 5;
    // holds a reference to x
    let y = &x;

    // we can assert x is equal to 5
    assert_eq!(5, x);
    // if we want to make an assertion about the value in y,
    //  we have to use the dereference operator to follow the 
    // reference to the value its pointing to
    assert_eq!(5, *y);
    
    let x = 5;
    // instance of box pointing to the value x
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
