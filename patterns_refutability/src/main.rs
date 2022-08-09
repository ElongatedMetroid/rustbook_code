// Patterns come in two forms, refutable and irrefutable. Patterns that
// will match for any possible value are passed are irrefutable, an 
// example would be x in the statement `let x = 5` because x matches 
// anything and therefore cannot fail to match.

// Patterns that can fail to match for some possible value are refutable
// an example would be Some(x) in the expression `if let Some(x) = a_val`
// beause if the value in a_val is None rather than Some, the Some(x)
// pattern will not match.

fn main() {
    // The code bellow will fail, this is because this case we are trying
    // to use a refutable pattern with let, when let only accepts 
    // irrefutable patterns, because if some_option_value was a None
    // value it would fail to match to the pattern Some(x).

    let some_option_value: Option<usize> = Some(1);
    // let Some(x) = some_option_value;

    // To fix this problem we can change the code that uses the pattern
    // instead of using let, we can use if let, then if the pattern does
    // not match the code will just skip the code in the curly braces.

    if let Some(x) = some_option_value {
        println!("{}", x);
    }

    // We can even do something like this, but rust complains that it
    // does not make sense to use if let with an irrefutable pattern.
    if let x = 5 {
        println!("{}", x);
    };
}
