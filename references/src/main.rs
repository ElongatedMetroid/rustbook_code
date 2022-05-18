fn main() {
    // reccomended to look at the ownership program before this
    // the issue when moving back owner ship from a function is that we have to return that value
    let s1 = String::from("Hello");
    let (s2, len) = calculate_len_bad(s1);

    let s3 = String::from("Hi world");
    // refer to value without taking ownership
    let len = calculate_len(&s3);
    println!("len = {}", len);

    // values that are references cannot be changed though, this is where mutable references come in play
    let mut s4 = String::from("Hello");
    change_str(&mut s4);
    println!("s4 = {}", s4);

    // you can only have a single mutable reference to a particular piece of data in a particular scope
    // This will fail
    // let r1 = &mut s4;
    // let r2 = &mut s4;
    // example: creating multiple mutable references
    let mut s5 = String::from("Hello");

    {   
        let r1 = &mut s5;
    } // r1 goes out of scope

    let r2 = &mut s5;

    // you can also NOT have a mutable reference while you have a immutable one
    // let r1 = &s; no problem
    // let r2 = &s; no problem
    // let r3 = &mut s; problem
}

fn calculate_len_bad(s: String) -> (String, usize) {
    let len = s.len();

    (s, len)
}

// takes in a REFERENCE to a string
fn calculate_len(s: &String) -> usize {
    s.len()
}

fn change_str(s: &mut String) {
    s.push_str(" brug");
}