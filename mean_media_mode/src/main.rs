use std::collections::HashMap;

fn main() {
    let nums = vec![4.0, 10.0, 7.0, 15.0, 2.0, 5.0];

    println!("mean of: {:?} is {}", nums, mean(&nums));

    println!("median of: {:?} is {}", nums, median(&nums));

    let nums = vec![1, 5, 6, 1, 7, 8, 5, 6, 7, 8, 9, 7, 1];
    println!("{}", mode(&nums));
}

fn mean(nums: &Vec<f64>) -> f64 {
    let mut mean = 0.0;

    for num in nums {
        mean += num; 
    }

    mean /= nums.len() as f64;

    mean
}

fn median(nums: &Vec<f64>) -> f64 {
    // copy passed in nums vector to a new mutable vector that we will sort
    let mut nums = nums.clone();

    nums.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // length so we dont have to repeatidly call the len method (cache len)
    let len = nums.len();

    // if is odd
    if len % 2 == 1 {
        // (n + 1) / 2
        nums.get(((len + 1) / 2) - 1).unwrap().to_owned()
    } else { // if is even
        // ((n/2) + (n/2) + 1) / 2
        ((nums.get((len/2) - 1)).unwrap().to_owned() + (nums.get(((len/2) + 1) - 1)).unwrap().to_owned()) / 2.0
    }
}

fn mode(nums: &Vec<isize>) -> isize {
    let mut map = HashMap::new();

    for num in nums.iter() {
        let count = map.entry(num).or_insert(0);

        *count += 1;
    }

    
}