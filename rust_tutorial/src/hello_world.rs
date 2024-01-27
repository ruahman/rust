#![allow(dead_code)]
#![allow(unused_imports)]

pub fn run() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        run()
    }
}
