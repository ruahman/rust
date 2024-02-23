use std::ops::Add;

// this function will work with any type that implements the Add trait
#[allow(dead_code)]
fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

#[derive(Debug)]
#[allow(dead_code)]
struct Point<T> {
    x: T,
    y: T,
}

#[allow(dead_code)]
pub fn run() {
    println!("{}", get_sum_gen(2, 2));
    println!("{}", get_sum_gen(2.2, 2.2));
    let a: Point<i32> = Point { x: 1, y: 2 };
    let b: Point<f32> = Point { x: 1.1, y: 2.2 };
    println!("{:?}", a);
    println!("{:?}", b);
}

// cargo test variables::tests -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generics() {
        run()
    }
}
