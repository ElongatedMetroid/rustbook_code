fn main() {
    // bad
    let mut s = String::from("Hello World");

    let word = first_word_bad(&s);

    s.clear();

    // word still has value 5 here but there is no string
    // that we could use the value 5 with word is now invalid

    // good
    let s = String::from("Hello world");

    // string slices that point to the 
    let hello = &s[0..5];
    let world = &s[6..11];

    let word = first_word(s.as_str());

    println!("first word of \"{}\" is \"{}\"", s, word);

    // now if we clear s (s.clear();) "word" will be invalid and that is what we want
}

// return index of the end of the first word
fn first_word_bad(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// returns a string slice that points to the first word
// takes in a slice as an argument so it can be used for more things
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
