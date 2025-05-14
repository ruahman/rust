#![allow(dead_code)]

// options are basicaly a enum of Some(v) or None
pub fn options() {
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

    let foo = None;
    // let num = foo.unwrap(); // this will cause a problem if the value is None

    let num = foo.unwrap_or(33);
    dbg!(num);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_options() {
        options();
    }
}
