#![allow(dead_code)]

// Import (via `use`) the `fmt` module to make it available.
use std::fmt;

pub fn display() {
    // Define a structure for which `fmt::Display` will be implemented. This is
    // a tuple struct named `Structure` that contains an `i32`.
    struct Structure(i32);

    // To use the `{}` marker, the trait `fmt::Display` must be implemented
    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    let s = Structure(3);
    println!("Display: {}", s);

    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let vec = &self.0;

            write!(f, "[")?;

            for (count, v) in vec.iter().enumerate() {
                // For every element except the first, add a comma.
                // Use the ? operator to return on errors.
                if count != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", v)?;
            }
            //
            // // Close the opened bracket and return a fmt::Result value.
            write!(f, "]")
        }
    }

    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_display() {
        display();
    }
}
