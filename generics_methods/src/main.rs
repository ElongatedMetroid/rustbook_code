struct Point<T, U> {
    x: T,
    y: U,
}

// (ignore U for this explaination)
// We have to declare T after impl so we can use it to specify we are implementing methods on the type Point<T>
// By declaring T as a generic type after impl Rust can identify that the type in angle brackets in Point is a generic type rather than a concrete type
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

// This means we can define methods only on certain instances (of types) of point
// for example this will only work on points with f64
impl Point<f64, f64> {
    fn distance_from_orgin(&self) -> f64 {
        // these mathmatical operations are only availible for floating points
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T, U> Point<T, U> {
    // This method takes another point as a parameter which might have different types than the self Point we call mixup on
    // Here the generic parameters T and U are declared after impl becuase they go with the struct definition,
    // the generic parameters V and W are declared after mixup because there only relavant to the method
    fn mixup<V, W> (self, other: Point<V, W>) -> Point<T, W> {
        Point { x: self.x, y: other.y }
    }
}

fn main() {
    let p = Point {x: 10.16, y: 5.5};

    println!("p.x = {}", p.x());

    println!("p's distance from the orgin {}", p.distance_from_orgin());

    let p1 = Point{x: 5, y: 10.5};
    let p2 = Point{x: 'L', y: "Ratio"};

    let p3 = p1.mixup(p2);

    println!("p3.x: {}, p3.y: {}", p3.x, p3.y);
}
