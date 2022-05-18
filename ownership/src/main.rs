fn main() {
    // create a new string variable (the String type is stored on the heap)
    let s1 = String::from("Hello");
    // assign s1 to s2, s2 now owns s1 and s1 is invalid
    // in other words copy the string data of s1 (ptr to loaction on heap, len, etc) to s2, and make s1 invalid
    let s2 = s1;

    // println!("s1 = {}", s1); <------- Compile time error since s1 was "moved" to s2
    println!("s2 = {}", s2);

    // if we want to make a new copy of s1 we would do a following
    let s1 = String::from("Hello");
    // explicily call the clone function to copy 
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // this behaviour does not occur when using variables such as ints (variables allocated on the stack)
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // s comes into scope
    let s = String::from("Hello");
    // s's value moves into the function and is no longer valid here
    takes_ownership(s);

    // x comes into scope
    let x = 5;
    // x would move into the function but since it is an integer (i32) it is copyed, so you can still use x afterwards
    makes_copy(x);

    // returning values can also transfer ownership

    // gives_ownership's return value is moved into s1
    let s1 = gives_ownership();
    println!("s1 = {}", s1);

    let s2 = String::from("Hello");

    // s2 is moved into takes_and_gives_back, takes_and_gives_back moves its return value to s3
    let s3 = takes_and_gives_back(s2);
    println!("s3 = {}", s3);

}

fn takes_ownership(some_string: String) {       // some_string comes into scope
    println!("some_string = {}", some_string);
} // some_string goes out of scope and drop is called (which drops the memory)

fn makes_copy(some_integer: i32) {              // some_integer comes into scope
    println!("some_integer = {}", some_integer);
} // some_integer goes out of scope. Nothing happens

// move its return value to the function that called it
fn gives_ownership() -> String {
    // some_string comes into scope
    let some_string = String::from("Hi");

    // some_string is returned and moves to the calling function
    some_string
}

// a_string comes into scope
fn takes_and_gives_back(a_string: String) -> String {
    // a_string is returned and moves out into the calling function
    a_string
}