// "global variable", in rust these are called static variables
// Static variables are similar to constants, static variables can
// only store refrences with the 'static lifetime, which means the
// rust compile can figure out the lifetime and we arent required
// to annotate it explicitly. Another difference is values in a 
// static variable have a fixed address in memory, using the value
// will always access the same data. Constants are allowed to
// duplicate thier data whenever they're used. One last difference
// is that static variables can be mutable, but accessing and 
// modifying mutable static variables is unsafe.
static HELLO_WORLD: &str = "Hello World!";
static mut COUNTER: u32 = 0;

fn main() {
    // ----- Dereferencing a raw pointer -----
    let mut num = 5;

    // No need to include the unsafe keyword here, we can create raw
    // pointers in safe code, we just cant't dereference raw 
    // pointers outside an unsafe block.

    // immutable raw pointer
    let r1 = &num as *const i32;
    // mutable raw pointer
    let r2 = &mut num as *mut i32;

    // You cannot dereference raw pointers in safe rust but you can
    // in unsafe rust
    unsafe {
        // Creating the pointer is not what does any harm, 
        // acessing the value it points at is where we might end up
        // dealing with an invalid value.
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
    }

    let address = 0x012345usize;
    // Create a raw poiter that pointers to an arbitrary place in
    // memory.
    let _r = address as *const i32;

    // ----- Calling unsafe functions or methods -----

    // In unsafe functions/methods the whole thing is an unsafe
    // block, no need to have an unsafe block inside the function
    unsafe fn dangerous() {}

    unsafe {
        // Unsafe functions/methods need to be called inside an
        // unsafe block
        dangerous();
    }

    // ----- Creating a safe abstraction over unsafe code -----

    // Just because a function contains unsafe code doesnt mean we
    // need to mark the entire function as unsafe. 

    // Take the method split_at_mut as an example
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // we might try to implement it ourselves with the following
    // fn split_at_mut(values: &mut [i32], mid: usize) 
    // -> (&mut [i32], &mut [i32]) {
    //     let len = values.len();
    
    //     assert!(mid <= len);
    
    //     (&mut values[..mid], &mut values[mid..])
    // }

    // But this will not work since Rust thinks we are borrowing
    // from the same slice twice. But we are borrowing different
    // parts of the slice, which is okay since the two slices do
    // not overlap.

    use std::slice;

    fn split_at_mut(values: &mut [i32], mid: usize) 
        -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        // Get a raw pointer to the slice
        let ptr = values.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            // Return a tuple of 2 elements
            (
                // First slice of split, ..mid
                slice::from_raw_parts_mut(ptr, mid),
                // Second slice of split, ptr.add calculates the
                // offset from a pointer, mid..len-mid(remaining 
                // length)
                slice::from_raw_parts_mut(ptr.add(mid), 
                                        len - mid),
            )
        }
    }

    let mut s = vec![1, 2, 3, 4, 5, 6, 7, 8];

    let (x, y) = 
        split_at_mut(&mut s, 4);

    println!("x: {:?}, y: {:?}", x, y);

    // ----- Using extern functions to call external code -----

    extern "C" {
        // abs function from the C stdlib
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -32 according to C: {}",
                abs(-3));
    }

    // ----- Call rust code from other languages -----

    // Tell the rust compiler not to change the function name
    #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }

    // ----- Accessing or modifying a mutable static variable -----

    println!("variable is {}", HELLO_WORLD);

    fn add_to_count(i: u32) {
        unsafe {
            // With data that is globaly accessable, it is difficult
            // to ensure no data races, this is why mutable static
            // variables are unsafe.

            // Where possible you should use thread-safe smart
            // pointers
            COUNTER += i;
        }
    }

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    // ----- Implementing an unsafe trait -----

    // A trait is unsafe when at least one of its methods has some
    // invariant that the compiler cant verify.

    unsafe trait Foo {

    }

    unsafe impl Foo for i32 {

    }

    // ----- Accessing fields of a union -----

    // The final action that only works with unsafe is accessing
    // fields of a union. Accessing union fields is unsafe because 
    // Rust can’t guarantee the type of the data currently being 
    // stored in the union instance

    #[repr(C)]
    union MyUnion {
        f1: u32,
        f2: f32,
    }

    let u = MyUnion { f1: 1 };
    let f = unsafe {
        u.f1
    };
}