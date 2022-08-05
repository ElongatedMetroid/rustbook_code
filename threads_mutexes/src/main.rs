// A mutex only allows one thread to access some data at any 
// given time in order to access the data in a mutex a thread 
// must first signal it want access to acquire the mutex's lock.
// The lock is a data structure that is part of the mutex that 
// keeps track of who currently has access to the given data.

// Mutex's have 2 rules
// 1. You must acquire the lock before using the data
// 2. When you are done with the data that the mutex guards, you
//    must unlock the data the mutex guards so other threads can
//    acquire the lock.

use std::{sync::{Mutex, Arc}, thread};

fn main() {
    // Create a new mutex holding the value 0
    let m = Mutex::new(5);

    {
        // Use the lock method to acquire the lock
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("{:?}", m);

    // ----- Sharing a mutex between multiple threads -----

    // To use counter in multiple threads we will need to wrap
    // it in a Arc<T>, Arc stands for Atomically Reference 
    // Counted, atomics work like primitive types but are safe
    // to share across threads
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    // Create 10 threads
    for _ in 0..10 {
        // Create a clone of the Arc pointer 
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        }); // num goes out of scope and releases the lock

        // Push all the handles for the threads to the handles
        // vector
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
