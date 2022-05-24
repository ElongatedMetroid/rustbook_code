fn main() {
    // Vectors store multiple values of the same type, vectors can be expanded

    // Vector that holds type i32
    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);

    println!("v: {:?}", v);

    // the vec macro will insert values into the vector
    let v = vec![1, 2, 3];

    println!("v: {:?}", v);

    // there is two ways to access a vectors data
    // causes program to panic when getting a non-existant element
    let second = &v[1];
    println!("{}", second);
    // returns None without panicing when trying to access a non-existant element
    let second = v.get(1);
    println!("{}", second.unwrap());
     
    // iterating over the values in a vector
    let mut v = vec![70, 123, 12, 32];

    // uses for loop to get immutable references to each element
    for i in &v {
        println!("{}", i);
    }

    // uses for loop to iterate over mutable references and add 50 to each
    for i in &mut v {
        // to change the value the mutable reference refers to we have to use the dereference operator to get the value of i
        *i += 50;
    }
}
