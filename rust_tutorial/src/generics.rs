#![allow(dead_code)]

use std::ops::Add;

// this function will work with any type that implements the Add trait
// Output = T means that the function will return the same type as the input
fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

#[derive(Debug)]
#[allow(dead_code)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x_val(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn x_float(&self) -> f32 {
        self.x
    }
}

// fn largest<T: PartialOrd>(list: &[T]) -> T {
//     let mut largest = list[0];
//
//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }

pub fn generics() {
    println!("{}", get_sum_gen(2, 2));
    println!("{}", get_sum_gen(2.2, 2.2));
    let a: Point<i32> = Point { x: 1, y: 2 };
    let b: Point<f32> = Point { x: 1.1, y: 2.2 };
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{}", a.x_val());
    println!("{}", b.x_float());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generics() {
        generics()
    }
}
