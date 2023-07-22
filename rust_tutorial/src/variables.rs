pub fn exec() {
    let x = "this should not change";
    // x = "cant't do this"
    println!("x: {}", x);

    #[allow(unused_assignments)]
    let mut age = 37;
    age = 40;
    println!("age: {}", age);

    // deconstruct
    let (name, age) = ("Brad", 37);
    println!("name: {}", name);

    println!("age: {}", age);

    // shadowing
    let y = 5;
    let y = y + 1;
    println!("y: {}", y);
    {
        let y = y * 2;
        println!("y: {}", y);
    }
    println!("y: {}", y);
}

// cargo test variables::tests -- --nocapture
#[cfg(test)]
mod tests {
    use super::exec;

    #[test]
    fn test_variables() {
        exec()
    }
}
