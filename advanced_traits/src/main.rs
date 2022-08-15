fn main() {
    // ----- Specifying Placeholder Types in Trait Definitions -----
    // ----- with Associated Types                             -----

    // Associated types connect a type placeholder with a trait
    // such that the trait method definitions can use these 
    // placeholder types in their signatures, the implmentor of a 
    // trait will specify the concrete type to be used instead of
    // the placeholder type for the particulat implmentation. This
    // allows us to define a trait that uses some types without 
    // needing to know exactaly what those types are until the trait
    // is implmented.

    // One example of a trait with an associated type is the
    // Iterator trait in the std library.

    // pub trait Iterator {
    //     type Item;
    
    //     fn next(&mut self) -> Option<Self::Item>;
    // }

    // The type Item is the type of the values the type implementing
    // Iterator trait is iterating over.

    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        } 
    }

    let counter = Counter::new();

    for count in counter {
        println!("count: {}", count);
    }

    // ----- Default generic type parameters and operator -----
    // ----- overloading                                  -----

    // When using generic types parameters, we can specify a default
    // concrete type for the generic types, this eliminates the need
    // for implementors of the trait to specify a concrete type if
    // the default type works.

    use std::ops::Add;

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32, 
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        /// Add the x values of two Point instances, and add the y
        /// values of two Point instances to create a new Point
        fn add(self, rhs: Self) -> Self::Output {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    // The default generic type in this code is within the Add trait
    // the definition is as follows.

    // trait Add<Rhs=Self> {
    //     type Output;

    //     fn add(self, rhs: Rhs) -> Self::Output;
    // }

    // The syntax of Rhs=Self is new, this is called default type
    // parameters. The Rhs generic parameter defines the type of
    // the rhs parameter in the add method
    // If we dont specify a concrete type for Rhs when we implement
    // the Add trait the type of Rhs will default to Self

    // When we implemented Add for Point, we used the default for
    // Rhs because we wanted to add two Point instances.

    // Lets now try implmenting the Add trait where we want to 
    // customize the Rhs type rather than using the default.

    #[derive(Debug)]
    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;
    
        fn add(self, rhs: Meters) -> Self::Output {
            Millimeters(self.0 + (rhs.0 * 1000))
        }
    }

    println!("{:?}", Millimeters(50) + Meters(5));

    // ----- Fully Qualified Syntax for Disambiguation: -----
    // ----- Calling Methods with the Same Name         -----

    // Rust does nothing to prevent you from having a method with
    // the same name as another traits method, nor does Rust 
    // prevent you from implementing both traits on one type.

    // When calling methods with the same name you will need to tell
    // Rust which one you want to use, consider bellow, there are
    // two traits named Pilot and Wizard, that both have a method
    // called fly, we implement both traits on a type Human, that
    // already has a method fly on it, each fly does something 
    // different.

    trait Pilot {
        fn fly(&self);
    }
    
    trait Wizard {
        fn fly(&self);
    }
    
    struct Human;
    
    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }
    
    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }
    
    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }

    let person = Human;
    // Default to calling the method that is directly implemented
    // on the type.
    person.fly();

    // To call the fly methods from either the Pilot trait or 
    // Wizard trait, we need to use more explicit syntax to specify
    // which fly method we mean.

    Pilot::fly(&person);
    Wizard::fly(&person);

    trait Animal {
        fn baby_name() -> String;
    }
    
    struct Dog;
    
    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }
    
    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }

    // Fails since Animal::baby_name() does not hae a self parameter
    // and there could be other types that implement the Animal
    // trait so Rust cant figure out what implementation of 
    // Animal::baby_name() we want.

    // println!("A baby dog is called a {}", Animal::baby_name());
    
    println!(
        "A baby dog is called a {}", 
        // Gives a type annotation within the angle brackets, which
        // indicates we want to call baby_name method from the 
        // Animal trait implemented on Dog, by saying that we want
        // to treat the Dog type as an Animal for this function call
        <Dog as Animal>::baby_name()
    );

    // This is also called fully qualified syntax, and generally is
    // defined as follows
    // <Type as Trait>::function(receiver_if_method, next_arg, ...);

    // ----- Using Supertraits to Require One Trait's ------
    // ----- Functionality Within Another Trait       ------

    // Sometimes you might write a trait definition that depends
    // on another trait: for a type to implement the first trait,
    // you want to require that type to also implement the second
    // trait.

    // You would do this so that your trait definition can make use
    // of the associated items of the second trait. The trait your 
    // trait definition is relying on is called a supertrait of your
    // trait.

    // The OutlinePrint trait will only work for types that also 
    // implement Display and also provide the functionality that
    // OutlinePrint needs

    use std::fmt;

    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            // We can use the to_string method that is automatically
            // implemented for any type that implements display.
            let output = self.to_string();
            let len = output.len();

            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    // Without implementing display for Point this will not compile
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    impl OutlinePrint for Point {}

    let point = Point { x: 0, y: 0 };
    point.outline_print();

    // ----- Using the Newtype Pattern to Implement External -----
    // ----- Traits on External Types                        -----

    // We are only allowed to implement a trait on a type if either
    // the trait or the type are local to our crate. 
    // It is possible to get around this restriction by using the
    // newtype pattern that involves createing a new type in a 
    // tuple struct.

    // Lets say we ant to implement Display on Vec<T>

    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    let w = Wrapper(vec![
            String::from("hello"), 
            String::from("world")
        ]
    );
    println!("w = {}", w);
}