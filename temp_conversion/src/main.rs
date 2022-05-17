use std::io;

fn main() {
    loop {
        println!("Enter a number to convert to fahrenheit and celsius");

        let mut temp_input = String::new();

        // get input for number to convert to celsius and fahrenheit
        io::stdin().read_line(&mut temp_input)
            .expect("Could not read line!");

        // convert string to f64
        let temp_input: f64 = match temp_input.trim().parse() {
            // if okay return f (number from the string) to temp_input
            Ok(f) => f,
            // if there were letters skip printing answers and ask again
            Err(_) => {
                println!("Please enter a number");
                continue
            }
        };

        println!("{}f = {}c", temp_input, to_celsius(temp_input));
        println!("{}c = {}f", temp_input, to_fahrenheit(temp_input));
    }

}

fn to_fahrenheit(celsius: f64) -> f64 {
    let fahrenheit = celsius * (9.0/5.0) + 32.0;

    fahrenheit
}

fn to_celsius(fahrenheit: f64) -> f64 {
    let celsius = (5.0/9.0) * (fahrenheit - 32.0);

    celsius
}
