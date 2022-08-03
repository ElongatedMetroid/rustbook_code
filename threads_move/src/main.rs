// Sometimes you might want to use data from a different thread inside
// a newly created thread. For this you can use move on the closure 
// within the thread::spawn assosiated function.

fn main() {
    // The following commented out section will not compile. This is 
    // because the closure uses v and it will capture v and make it
    // part of the closures environment. And since rust infers how to
    // capture v and println! only needs a reference to v the closure
    // tries to borrow v.
    // But since rust does not know how long the spawned thread will run
    // it doesnt know if the reference to v will always be valid.

    // let v = vec![1, 2, 3];

    // let handle = thread::spawn(|| {
    //     println!("Here's a vector: {:?}", v);
    // });

    // drop(v); // <--- Not required for a compile time error

    // handle.join().unwrap();

    // The following code will indeed work and function the same since
    // before the closure we add the move keyword, we force the closure
    // to take ownership of the values its using, rather than having 
    // rust infer that it should borrow the values
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // drop(v); // <--- Compile time error; v was moved into the closure

    handle.join().unwrap();
}
