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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_options() {
        run();
    }
}
