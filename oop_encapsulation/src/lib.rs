// A common aspect of OOP is encapsulation, which means that the
// implementation details of an object aren't accessible to code
// using that object. Therefore the only way to interact with an
// object is through its public API, code using the object 
// should not be able to reach into the objects internals and
// change data or behavior directly. This allows the programmer
// to change and refactor an objects internals without needing
// to change the code that uses that object.

// Encapsulation can be accived through the pub keyword

// Other code can use this struct but the fields within remain
// private. This is important because we want to ensure that
// whenever a value is added or removed, the average is updated
pub struct AveragedCollection {
    // Thie list is private so code using an AveragedCollection
    // cannot directly modify it so average does not become out
    // of sync
    list: Vec<i32>,
    // The average will not have to be computed on demand 
    // everytime someone needs it, AveragedCollection will 
    // cache the calculated average for us.
    average: f64,
}

impl AveragedCollection {
    /// Add an element to the lsit
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }
    /// Remove an element from the list
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();

        match result {
            // Return the last element in the vector and
            // recalculate the average.
            Some(value) => {
                self.update_average();
                Some(value)
            }
            // List was empty
            None => None,
        }
    }
    /// Return the cached average
    pub fn average(&self) -> f64 {
        self.average
    }
    /// Private method to update the average
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }

}