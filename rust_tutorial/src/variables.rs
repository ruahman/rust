use std::io;

#[allow(unused_assignments)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]

pub fn exec() {

    //// variables
    // once a variable is set it does not change
    let x = "this should not change";
    // x = "cant't do this"
    println!("x: {}", x);

    //// mutable variables
    let mut age = 37;
    age = 40;
    println!("age: {}", age);

    let mut name = String::from("Diego");
    name = String::from("Brad");

    // read line
    println!("Enter your name: ");
    io::stdin().read_line(&mut name).expect("Failed to read line");

    let geeting = "hello world";
    println!("name: {}: {}", name.trim(), geeting);

    //// constants
    // this is assigned at compile time
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.14;

    //// shadowing
    let age = "42";
    // this shadows the previous age variable
    let mut age: u32 = age.trim().parse().expect("age was not a number");
    age = age + 1;
    println!("age: {}, mil: {}, pi: {}", age, ONE_MIL, PI);

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
    fn test_exec() {
        exec()
    }
}
