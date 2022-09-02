pub fn demo() {
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
