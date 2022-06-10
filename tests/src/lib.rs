#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value > 100 || value < 1 {
            panic!("Value must be between 1 and 100, got {}", value);
        }
            
        Guess {value}
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

fn greeting(name: &str) -> String {
    String::from("Hello!")
}

#[cfg(test)]
mod tests {
    // since tests is a module is an inner module we need to bring the code in the outer module into the inner module
    use super::*;

    #[test]
    // indicates that the function below SHOULD panic for it to pass
    #[should_panic] 
    fn greater_than_100() {
        // the new method panics when a value higher than 100 is entered
        Guess::new(200);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7};
        let smaller = Rectangle {length: 5, width: 1 };

        // if the parameter is true assert will do nothing, if it is false assert will call panic
        assert!(larger.can_hold(&smaller));
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Jimmy");
        assert!(
            result.contains("Jimmy"),
            "Greeting did not contain name, value was '{}'", result
        );
    }

    #[test] // This attribute indicates the function below is a test function
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}
