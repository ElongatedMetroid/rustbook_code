pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {

            }
        }
    }
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// the use keyword can also be used to bring enum variants into scope
use TrafficLight::{Red, Yellow};
// You can use Globs to bring all the items in
use TrafficLight::*;

use a::series::of;

fn main() {
    // instead of doing:
    // a::series::of::nested_modules();
    // (which can get quite long)
    // you can use the use keyword to bring modules of the function you want to call into scope
    // in this case the line 
    // use a::series::of
    // brings of into scope
    // making a shorter way to call the function
    of::nested_modules();

    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
}
