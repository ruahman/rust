pub fn exec() {
    println!("Hello, world!");
}

// cargo test hello_world::tests -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exec() {
        exec()
    }
}
