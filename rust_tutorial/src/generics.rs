use std::ops::Add;

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
pub fn exec() {
    println!("{}", get_sum_gen(2, 2));
    println!("{}", get_sum_gen(2.2, 2.2));
    let a = Point { x: 1, y: 2 };
    println!("{:?}", a);
}

// cargo test variables::tests -- --nocapture
#[cfg(test)]
mod tests {
    use super::exec;

    #[test]
    fn test_exec() {
        exec()
    }
}
