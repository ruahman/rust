#[allow(unused_assignments)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
use std::mem;

// this is computed at compile time
const MEANING_OF_LIFE: u8 = 42; // no fixed address

// static variables have fixed address
static Z: i32 = 123; // this has an address

pub fn run() {
    let a: u8 = 123; // unsigned 8 bits
    println!("a: {}", a); // immutable

    // can't do this
    // a = 456;

    let mut b: i8 = 0; // signed 8 bits mutable
    println!("b: {}", b);
    b = 42;
    println!("b: {}", b);

    // type inference
    let c = 123456789; // 32-bit signed i32
    println!("c: {}, size: {} bytes", c, mem::size_of_val(&c));

    // usize isize default size of your system
    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!(
        "z: {}, takes up {} bytes, {}-bit OS",
        z,
        size_of_z,
        size_of_z * 8
    );

    // char
    let d: char = 'x';
    println!("d: {}, size: {} bytes", d, mem::size_of_val(&d));

    // f32 f64
    let e: f32 = 2.5;
    println!("e: {}, size: {} bytes", e, mem::size_of_val(&e));

    // boolean
    let g: bool = false;
    println!("g: {}, size: {} bytes", g, mem::size_of_val(&g));

    // scope and shadowing
    let a = 123;
    {
        let b = 456;
        println!("inside, b: {}", b);

        // this shadows the previous a variable
        let a = 777;
        println!("inside, a: {}", a);
    }
    println!("outside, a: {}", a);

    // constants
    println!("MEANING_OF_LIFE: {}", MEANING_OF_LIFE);
    println!("Z: {}", Z);

    // stack and heap
    // stack is allocated when calling the function, it's freed when the function returns
    // when you declare a variable, it's stored on the stack, it's calulated ahead of time.
    // it checks the function to find out how much space it needs and then it allocates that space

    // heap is allocated when we need it, it's freed when we explicitly return it to the OS,
    // heep returns a pointer to the memory address

    struct Point {
        x: f64,
        y: f64,
    }

    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // stack allocated
    let p1 = origin();
    println!("p1 takes up {} bytes", mem::size_of_val(&p1));

    // heap allocated
    let p2 = Box::new(origin());
    // p2 is just a pointer so it's smaller
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));

    // // deconstruct
    // let (name, age) = ("Brad", 37);
    // println!("name: {}", name);
    //
    // println!("age: {}", age);
}

// cargo test variables::tests -- --nocapture
#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_variables() {
        run()
    }
}
