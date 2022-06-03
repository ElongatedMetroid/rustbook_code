// struct that can hold x and y cordinate of any type
#[derive(Debug)]
struct PointSingleType<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct PointMultiType<T, U> {
    x: T,
    y: U,
}

fn main() {
    // since the structure used here only uses one generic type you cant have for example x be an int while y is a float
    let integer = PointSingleType{ x: 5, y: 10 };
    let float = PointSingleType{ x: 1.5, y: 2.3 };
    
    println!("integer {:?}, float {:?}", integer, float);

    // with this type the values can both be different types at the same time since it takes 2 generics
    let integer = PointMultiType{ x: 5, y: 10 };
    let float = PointMultiType{ x: 1.5, y: 2.3 };
    let int_and_float = PointMultiType{x: 1.5, y: 3};
    
    println!("integer {:?}, float {:?}, int and float {:?}", integer, float, int_and_float);
}
