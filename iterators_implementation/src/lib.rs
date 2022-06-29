struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    // set the associated Item type for the iterator to a u32,
    // the iterator will return u32 values
    type Item = u32;

    // implement a counter that can only go up to 5
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
    // take the values produced by an instance of Counter, 
    // pair them with values produced by another counter 
    // instance after skipping the first value
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                                 // multiply each pair together 
                                 .map(|(a, b)| a * b)
                                 // keep only the results that are divisible by 3
                                 .filter(|x| x % 3 == 0)
                                 // and add the resulting values together
                                 .sum();

    assert_eq!(18, sum);
}