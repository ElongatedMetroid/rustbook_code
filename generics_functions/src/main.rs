// the function itself isnt that bad but we are only able to
// get the largest of lists that contain i32's and no other
// type
fn largest_bad(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &number in list.iter() {
        if number > largest {
            largest = number;
        }
    }

    largest
}

// this function is better because it can get the largest value 
// out of any type of slice

// The greater than and less than operators are defined in the standard 
// library with PartialOrd so we need T to implement that so we can compare number > largest
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
