fn main() {
    let mut number = 10;
    while number != 0 {
        println!("{}", number);

        number -= 1;
    }

    for number in (1..4).rev() {
        println!("{}", number);
    }

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    
    while index < 5 {
        println!("index {}: {}", index, a[index]);

        index += 1;
    }

    for element in a.iter() {
        println!("value {}", element);
    }

    // loop {
    //     println!("Again!");
    // }
}
