// constant value and replaces it with the value when every it's called,
// makes sence that this is global
const PI: f64 = 3.14;

// constant memory address then anyone can get
static mut Z: i32 = 333;

pub fn demo() {
    // variable are unmutable by default
    let x = 1;
    let y = 3.12;
    let xi: i64 = 44;
    println!("{}{}{}", x, y, xi);

    // if you want variable to be mutable
    let mut b: i8 = 0;
    println!("{}", b);
    b = 75;
    println!("{}", b);

    // constants
    println!("{}", PI);
    // access to a global peice of memory where a buch of thread can acccess,
    // is considered unsafe.
    unsafe {
        Z = 555;
        println!("{}", Z);
    }
}
