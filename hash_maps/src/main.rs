// Hash maps arent as common as other data types so you have to bring it into scope
use std::collections::HashMap;

fn main() {
    // hash maps store a key that gives you a value
    
    // define a new empty hash map with new();
    let mut scores = HashMap::new();

    // team Blue (key) has a score of 10 (value)
    scores.insert(String::from("Blue"), 10);
    // team Red (key) has a score of 50 (value)
    scores.insert(String::from("Red"), 50);

    // you can use the collect method on a vector of tuples, the collect method gathers data into a number of collection types (including hash maps)
    let teams = vec![String::from("Blue"), String::from("Red")];
    let initial_scores = vec![10, 50];

    // turn teams, and initial_scores into a vector of tuples (where blue is paired with 10, etc), then use the collect method to turn that vector of tuples into a hash map
    // the HashMap<_, _> is required because the collect method needs to know what collection (in this case a HashMap) you want to convert to
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // to get values you can provide the key to the get method
    // Returns an Option enum, if there is no value for a key it will return None
    let score = scores.get(&String::from("Blue"));

    println!("score: {}", score.unwrap());

    // loop through all elements in a hash map
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // overwriting a value, overwrites value for blue
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // check whether a key has a value and if it does not insert a value for it
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 0);

    // the or_insert method returns a mutable reference to the value for the corresponding entry key
    // if that key does not exist it inserts the parameter as the new value for this key and returns a mutable reference to the new key
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(50);

    println!("{:?}", scores);

    // updating value based on the old value
    let keys = "hello world wonderful world";

    let mut map = HashMap::new();

    // set key equal to each word of text (one at a time) that are seperated by a space
    for key in keys.split_whitespace() {
        let count = map.entry(key).or_insert(0);
        // the or_insert method returns a mutable reference to the value of the key
        // so we can dereference that value and modify it, in this case add 1
        *count += 1;
    }
    
    println!("{:?}", map);
}
