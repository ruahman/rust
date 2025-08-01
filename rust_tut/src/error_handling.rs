#![allow(dead_code)]

use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub fn run() {
    let path: &str = "error_handling.txt";
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::PermissionDenied => {
                panic!("Permission denied")
            }
            _ => panic!("Problem creating the file: {:?}", error),
        },
    };
    write!(output, "hello from error handling").expect("cannot write to file");

    let input = File::open(path).unwrap();
    let buffer = BufReader::new(input);
    for line in buffer.lines() {
        println!("{}", line.unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_error_handling() {
        run();
    }
}
