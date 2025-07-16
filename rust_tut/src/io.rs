#![allow(dead_code)]

use std::io;

pub fn stdio() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line"); // if it fails, it will throw an error with this message

    input = match input.trim().parse() {
        Ok(input) => input,
        Err(_) => "Error".to_string(),
    };
    input
}
pub fn run() {
    println!("***** io demo *****");

    println!("Please enter something");
    let res = stdio();

    println!("You entered: {res}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_io() {
        run();
    }
}
