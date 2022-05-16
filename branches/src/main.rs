fn main() {
    let num = 6;

    if num > 4 {
        println!("Condition is true");
    } else {
        println!("Condition was false");
    }

    if num != 0 {
        println!("Num is not equal to zero");
    }

    if num % 4 == 0{
        println!("Num is divisible by 4");
    } else if num % 3 == 0 {
        println!("Num is divisible by 3");
    } else if num % 2 == 0{
        // even though 6 is divisible by 2 rust only executes the first block of the first true condition
        println!("Num is divisible by 2");
    } else {
        println!("Num is not divisible by 4, 3, or 2");
    }

    // if is an expression so we can bind a variable to it
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is {}", number);
}
