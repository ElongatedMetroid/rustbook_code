use std::io;

fn main() {
    loop {
        println!("Enter the fibonacci number you would like to calculate");

        let mut num_input = String::new();

        io::stdin().read_line(&mut num_input)
            .expect("Error in getting input!");
        
        let num_input: u64 = match num_input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Please enter a number");
                continue
            },
        };

        println!("{}th fibonacci num = {}", num_input, fibonacci_num(num_input));        
    }
}

fn fibonacci_num(n: u64) -> u64 {
    let mut a = 1;
    let mut b = 1;

    // loop n times
    for _ in 1..n {
        // store a in temp variable
        let old = a;
        // set a to b
        a = b;
        // add old to b (b is the sum of a + old)
        b += old;
    }

    // return b once you have looped through n times
    b
}