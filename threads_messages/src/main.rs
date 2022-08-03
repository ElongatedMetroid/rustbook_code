// One popular approah to ensure safe concurrency is message passing,
// where threads communicate by sending each other messages that contain
// data.
// A channel in programming can be though of as being a stream of water
// that goes a single direction, if you put something on the river it
// will travel down stream.
// A channel has two halves: a transmitter and receiver. The transmitter
// can be though of as the top of the stream, and the reciver can be
// though of as the bottom of the stream where the items going down the
// stream will end up.
// One part of the code calls the methods on the transmitter with the 
// data we want to send, and another part checks the reciving end for
// arriving messagges.

// This program will have one thread to generate values and send them
// down a channel, and another thread will receive these values and
// print them out.

use std::{sync::mpsc, thread, time::Duration};

fn main() {
    // Create a channel and assign the two halves tx and rx
    // mpsc stands for "Multiple Producer, Single Consumer", meaning
    // multiple transmitters with one reciver is possible
    let (tx, rx) = mpsc::channel();
    // tx - the transmitter, rx- the receiver.

    // Clone tx
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("Hello"),
            String::from("World"),
            String::from("This is from"),
            String::from("A new thread"),
        ];

        // Send a value then wait 1 second to send the next
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("That was easy"),
            String::from("but can you handle"),
            String::from("TWO TRANSMITTERS!!!??!"),
            String::from("Wait I forgot you have the power of rust"),
        ];

        // Send a value then wait 1 second to send the next
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Foreach value we have recived print it
    for received in rx {
        println!("Got: {}", received);
    }

    // the recv() method will block the main threads execution and wait
    // until a value is sent down the channel. There is also a try_recv()
    // method that doesnt block execution but instead will immediately
    // will return a Result<T, E>, an Ok() value holding the message if
    // any were recived, and an Err value if there aren't and messages
    // at the time.
    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);
}
