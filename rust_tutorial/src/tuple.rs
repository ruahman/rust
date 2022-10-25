pub fn demo() {
    println!("***** tuple demo *****");

    //// simple example
    let person: (&str, &str, i8) = ("diego", "vila", 40);
    println!("{}:{}:{}", person.0, person.1, person.2);

    //// deconstuct tuples
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("tup: {x},{y},{z}");

    //// access tuple member directly
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("{five_hundred},{six_point_four},{one}");
}
