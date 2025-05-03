#[allow(clippy::unnecessary_literal_unwrap)]
pub fn run() {
    // it's either someting or nothing
    let r = Some(32);

    match r {
        Some(x) => println!("{:?}", x),
        None => println!("nothing"),
    }

    let r2 = Some(99);

    if let Some(z) = r2 {
        println!("{:?}", z);
    }

    let r3 = Some(99);
    let x = r3.unwrap();
    println!("{:?}", x);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_options() {
        run();
    }
}
