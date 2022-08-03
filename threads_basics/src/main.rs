use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn( || {
        for i in 1..10 {
            println!("i is {} in the spawned thread", i);
            // Force the thread to stop its execution for a short
            // duration to allow the main thread to run
            thread::sleep(Duration::from_millis(1));
        }
    });    

    for i in 1..5 {
        println!("i is {} in the main thread", i);
        // Force the thread to stop its execution for a short
        // duration to allow the other thread to run
        thread::sleep(Duration::from_millis(1));
    }

    // The new thread above will (and and other new threads)
    // will be stopped when the main thread ends even if the
    // new thread has not finished running.
    // To prevent that from happening we can use the join
    // method to wait for the spawned thread to finish before
    // the main thread exits
    handle.join().unwrap();
}
