#[derive(Debug, PartialEq)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // into_iter will create an iterator that takes ownership of the vector
    shoes.into_iter()
        // The filter method takes a closure that takes each item from 
        // the iterator and returns a boolean, if the value is true
        // the value will be included in the iterator produced by filter
        .filter(|s| s.size == shoe_size)
        // Collect all the values (shoes of your size) into a Vector of Shoes
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe { size: 10, style: String::from("tennis")},
            Shoe { size: 9, style: String::from("sandal")},
            Shoe { size: 10, style: String::from("boot")},
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            vec![
                Shoe { size: 10, style: String::from("tennis")}, 
                Shoe { size: 10, style: String::from("boot")},
            ], 
            in_my_size
        );   
    }
}
