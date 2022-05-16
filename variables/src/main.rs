fn main() {
    // x has to be mutable for it to change
    let mut x = 5;
    println!("x = {}", x);
    x = 6;
    println!("x = {}", x);

    // constants cannot be changed, and must be declared before runtime
    const PI:f64 = 3.14595;
    const MAX_HEALTH:u32 = 1_000;

    println!("PI = {}, MAX_HEALTH = {}", PI, MAX_HEALTH);

    // shadowing, most times used for changing the type of the variable
    let y = 5;
    let y = y + 2;
    let y = y * 2;

    println!("y = {}", y);

    // convert str to integer
    let spaces = "      ";
    let spaces = spaces.len();

    println!("spaces = {}", spaces);

    // tuples are like arrays but they can store many different data types
    let tup = (70, 8.75, true);

    // a = tup.0, b = tup.1, c = tup.2
    let (a, b, c) = tup;

    println!("a = {}, b = {}, c = {}", a, b, c);

    println!("tup.0 = {}, tup.1 = {}, tup.2 = {}", tup.0, tup.1, tup.2);

    // arrays, an arrays size in rust cannot be exteneded
    let months = ["January", "Febuary", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    println!("months[0] = {}", months[0]);
}
