extern crate add_one;

fn main() {
    let x = 2;
    println!("{} plus one is {}", x, add_one::add_one(x));
}
