fn how_many(x: i32) -> &'static str {
    match x {
        0 => "no",
        1 | 2 => "one or two",
        9..=11 => "lots of",
        12 => "a dozen",
        _ if (x % 2 == 0) => "some",
        _ => "a few",
    }
}

pub fn run() {
    println!("Pattern Matching from diego");
    for x in 0..13 {
        println!("How many? {}", how_many(x));
    }

    let point = (3, 4);
    match point {
        (0, 0) => println!("origin"),
        (0, y) => println!("x axis, y = {}", y),
        (x, 0) => println!("y axis, x = {}", x),
        (x, y) => println!("({}, {})", x, y),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pattern_matching() {
        run();
    }
}
