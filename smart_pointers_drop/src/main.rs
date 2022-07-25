struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    /// Code you want to go when an instance of CustomSmartPointer goes
    /// out of scope.
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}

fn main() {
    let p = CustomSmartPointer { data: String::from("Hello world") };
    let q = CustomSmartPointer { data: String::from("Goodbye world") };

    println!("Smart pointers created!");

    // If you want to drop a pointer befoer the end of there scope
    // you cannot just call the drop method manually. 
    // q.drop() <---- ERROR
    // If this were allowed it would cause a double free error since
    // it will be tryed to drop at the end of main. There is a way
    // to drop smart pointers early though, by using std::mem::drop

    drop(q);

    println!("CustomSmartPointer `q` dropped early");

} // The pointer will be dropped here since this is the end of there scope
