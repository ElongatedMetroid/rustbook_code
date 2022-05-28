fn main() {
    let nums = vec![5.0, -10.0, 50.0, 10.0];

    println!("mean of: {:?} is {}", nums, mean(&nums));
}

fn mean(nums: &Vec<f64>) -> f64 {
    let mut mean = 0.0;

    for num in nums {
        mean += num; 
    }

    mean /= nums.len() as f64;

    mean
}

fn median