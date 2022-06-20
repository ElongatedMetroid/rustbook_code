fn main() {
    // closures can also take advantage of stuff like local variables
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));

    // you can use the move keyword to transfer ownership of the values it uses in the environment
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    // This println! will fail since the ownership of x was transfered to the closure
    // println!("Cant use x here {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
