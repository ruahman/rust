pub fn run() {
    println!("Hello, formatting!");

    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    println!("Base 10:               {}", 69420);
    println!("Base 2 (binary):       {:b}", 69420);
    println!("Base 8 (octal):        {:o}", 69420);
    println!("Base 16 (hexadecimal): {:x}", 69420);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_formatting() {
        run();
    }
}
