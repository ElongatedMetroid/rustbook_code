fn main() {
    let num_list = vec![34, 64, 243, 3455, 90, 3242340, 213434214, 41423434];
    let largest = largest_bad(&num_list);

    println!("largest: {}", largest);

    let largest = largest(&num_list);

    println!("largest: {}", largest);

    let char_list = vec!['A', 'B', 'D', 'Z'];
    let largest = largest(&char_list);

    println!("largest: {}", largest);
}

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
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &number in list.iter() {
        if number > largest {
            largest = number;
        }
    }

    largest
}