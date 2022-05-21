#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// implements a function within the context of Rectangle
impl Rectangle {
    // &self is the same as &Rectangle, self is used for methods
    // since we dont want to take ownership we use a reference just like we would with any other function (instead in this case its a method)
    // you can use a mutable reference aswell (just like a normal function!)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // self always comes first
    fn can_hold(&self, other_rect: &Rectangle) -> bool{
        self.width > other_rect.width && self.height > other_rect.height
    }

    // associated functions do not take in self as a parameter
    // there called this because they are still accosiated with the struct, and they are not methods because they dont have an instance of the struct to work with
    // String::from is one example of an associated function 
    // associated functions usually return a new instance of the struct
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size}
    }
}
// multiple impl blocks are aloud, not used in this example though

fn main() {
    let rect = Rectangle {width: 30, height: 50};

    // {:?} tells println! to use the debug format+
    // {:#?} will output a 'prettier' output when displaying debug output
    println!("rect is: {:#?}", rect);

    println!("rect area is: {}", rect.area());

    let rect1 = Rectangle {width: 20, height: 40};
    let rect2 = Rectangle{width: 40, height: 60};

    println!("Can rect hold rect1? {}", rect.can_hold(&rect1));
    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));

    let square = Rectangle::square(5);
    println!("Creating a square: {:#?}", square);
}
