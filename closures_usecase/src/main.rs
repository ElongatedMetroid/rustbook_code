use std::collections::HashMap;
use std::thread;
use std::time::Duration;

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    values: HashMap<u32, u32>,
}

impl<T> Cacher<T> 
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher { calculation, values: HashMap::new() }
    }

    fn value(&mut self, arg: u32) -> u32 {
       if self.values.get(&arg).is_none() {
            // insert the value of the calculation at the key for its argument
            self.values.entry(arg).or_insert((self.calculation)(arg));
            // its OK to unwrap here since we know there is a value
            *self.values.get(&arg).unwrap()
        } else {
            *self.values.get(&arg).unwrap()
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}

fn generate_workout(intensity: u32, random_number: u32) {
    // contains the definition of an anonymous function (not the resulting value of calling it)
    let mut simulated_expensive_closure = Cacher::new( |num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    } );

    if intensity < 25 {
        println!(
            "Today do {} pushups",
            simulated_expensive_closure.value(intensity)
        );

        println!(
            "Now do {} curl-ups",
            simulated_expensive_closure.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Stay hydrated");
        } else {
            println!(
                "Today, run for {} minutes",
                simulated_expensive_closure.value(intensity)
            );
        }
    }
}