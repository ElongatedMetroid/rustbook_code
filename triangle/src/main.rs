use std::io;

fn main() {
    loop {
        let mut input = String::new();

        println!("Enter the width of the triangle (type: \"exit\" if you would like to exit)");

        io::stdin().read_line(&mut input)
            .expect("Invalid input");

        if input.trim() == "exit" {
            println!("Exiting...");
            return;
        }

        let input: u16 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };

        let mut buf = String::new();

        for _ in 0..input {
            buf.push('*');
            println!("{}", buf);
        }
    }
}
