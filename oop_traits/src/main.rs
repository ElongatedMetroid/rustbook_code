use oop_traits::{Draw, Screen, Button};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing a select box");
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            // Add our own custom SelectBox component
            // this works since out SelectBox trait
            // implements the Draw trait, which means
            // it implements the draw method
            Box::new(SelectBox { 
                width: 75, 
                height: 10, 
                options: vec![
                    String::from("Yes"),
                    String::from("No"),
                ], 
            }),
            Box::new(Button { 
                width: 50, 
                height: 10, 
                label: String::from("Okay"), 
            }),
            // If we try to add a component that does
            // not implement the Draw trait it will
            // give us a compile time error
            
            // Fails since Screen cant call draw on
            // String
            // Box::new(
            //     String::from("Helllo")
            // ),
        ],
    };
    // Draw all the components
    screen.run();
}