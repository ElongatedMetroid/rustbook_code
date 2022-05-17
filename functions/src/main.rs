fn main() {
    // Statments are instructions that perform an action and do not return a value
    // Expressions evauluate to something (return a value)
    let a = {
        let x = 3;
        // return x + 1 to a
        x + 1
    };

    another_function(a, 80);

    let b = return_five();

    println!("b = {}", b);

    let c = plus_one(2);

    println!("c = {}", c);
}

fn another_function(x: i32, y: i32) {
    println!("Another function {}, {}", x, y);
}

fn return_five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}