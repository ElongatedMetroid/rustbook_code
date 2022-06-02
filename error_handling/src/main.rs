use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};

fn main() {
    // causes program to panic
    // panic!("Crash and burn");

    // let v = vec![1, 2, 3];

    // causes program to panic
    // v[99];

    // most errors are not serious enough to require the program to stop entirly and our program can handle these errors
    // how can we do this?

    // Result enum
    // the Result enum contains Ok(T) and Err(E) (T and E being types), Ok being success and Err being Error
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,

        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Tried to create file but there was a problem: {:?}", e);
                }
            }
        },

        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error);
        }
    };

    // you can also use the unwrap and expect methods

    // unwrap will panic on error
    let f = File::open("Hello.txt").unwrap();

    // expect is the same as unwrap but you are able to provide and error message
    let f = File::open("Hello.txt").expect("Failed to open Hello.txt");


}

// returns a Result<T, E>, T being a String, E being an io::Error
// if this function succeeds it will return the username this function read from a file
// if this function runs into an error, the code that calls this will recive an Err value that holds an instance of io::Error
fn read_username_from_file_bad() -> Result<String, io::Error> {
    let f = File::open("username.txt");

    let mut f = match f {
        Ok(file) => file, 
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// this function could be much simpler though...
fn read_username_from_file() -> Result<String, io::Error> {
    // the ? operator is functionaly the same as using a match statment, it will return the Err value on fail
    let mut f = File::open("username.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}