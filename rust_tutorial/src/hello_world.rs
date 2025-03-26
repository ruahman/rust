#![allow(dead_code)]

pub fn hello_world() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        hello_world();
    }
}
