#[allow(dead_code)]
#[derive(Debug)]
struct Tuples {
    person: (&'static str, &'static str, i8),
    x: i32,
    y: f64,
    z: i32,
}
fn tuples() -> Tuples {
    let person: (&str, &str, i8) = ("diego", "vila", 41);
    println!("{}:{}:{}", person.0, person.1, person.2);

    //// deconstuct tuples
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    return Tuples { person, x, y, z };
}

pub fn demo() {
    println!("--- tuple demo ---");

    println!("tuple: {:?}", tuples());
}

#[cfg(test)]
mod tuples_tests {
    use super::*;

    #[test]
    fn test_tuples() {
        let res = tuples();
        assert_eq!(res.person.0, "diego");
        assert_eq!(res.person.1, "vila");
        assert_eq!(res.person.2, 41);
        assert_eq!(res.x, 500);
        assert_eq!(res.y, 6.4);
        assert_eq!(res.z, 1);
    }
}
