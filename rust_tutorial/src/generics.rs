use std::ops::Add;

fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

#[derive(Debug)]
#[allow(dead_code)]
struct Point<T> {
    x: T,
    y: T,
}

pub fn demo() {
    println!("{}", get_sum_gen(2, 2));
    let a = Point { x: 1, y: 2 };
    println!("{:?}", a);
}
