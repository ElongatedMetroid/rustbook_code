fn main() {
    let mut output = String::new();
    let s = String::from("first apple");
    let mut input = s.chars().peekable();

    // loop through each character of input string
    while let Some(c) = input.next() {
        // create suffix
        let suffix = match c {
            // is vowel
            'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => {
                // push first character into output since words that start with vowels only
                // have one change, the -hay suffix
                output.push(c);
                // return -hay to suffix
                String::from("-hay")
            } 
            // is not a vowel (but still is a letter)
            'a'..='z' | 'A'..='Z' => {
                // dont push since words that dont start with vowels move there first word to 
                // the beggining of the suffix
                // return sufix with the 1st character + ay
                format!("-{}ay", c)
            }
            // either white space or symbol
            _ => {
                // push the symbol to the output (unchanged)
                output.push(c);
                // skip this itteration 
                continue
            }
        };

        // sufix is now generated 

        // look through input (peek returns a value to the next but does not advance the iterator)
        while let Some(&c) = input.peek() {
            // this match statment causes loop to go until it hits whitespace or a symbol
            match c {
                // push char if it is a letter
                'a'..='z' | 'A'..='Z' => {
                    // call next first because the first char was already managed above
                    input.next();
                    output.push(c);
                }

                // break from loop if we find symbols or white space
                _ => break,
            }
        }

        // update output
        output = format!("{}{}", output, suffix);
    }

    println!("{}", output);
}
