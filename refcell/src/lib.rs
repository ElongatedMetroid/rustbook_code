pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T> 
    where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64; 
        
        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            self.messenger.send("Warning: You have used up over 75% of your quota");
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            self.messenger.send("Urgent Warning: You have used up over 90% of your quota");
        } else if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    struct MockMessenger {
        // We cant just use a Vec<String> since the send method in the Messenger
        // trait does not take a mutable reference to self, it takes an immutable
        // reference. By putting the Vector of strings inside a RefCell means we
        // can modify sent_messages, by using the borrow_mut method
        
        // In other words, RefCell's allow for interior mutability

        // Borrowing rules still apply to RefCells! 
        // At any given time you can have either but not both of the following:
        // one mutable reference or any number of immutable references
        // RefCels are just checked at runtime and not compile time
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            // get the data inside the RefCell mutably and push a String to it
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // assert the length of the value inside the RefCell 
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}