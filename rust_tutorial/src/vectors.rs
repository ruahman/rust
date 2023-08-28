#[allow(unused_variables)]
#[allow(dead_code)]
pub fn exec() {
    //// vector

    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1, 2, 3];
    vec2.push(4);
    println!("fst: {}", vec2[0]);
    match vec2.get(1) {
        Some(x) => println!("snd: {}", x),
        None => println!("none"),
    }

    for x in &vec2 {
        println!("{}", x);
    }

    // vec is a mutable array
    let mut numbers: Vec<i32> = vec![1, 2, 3];

    numbers.push(5);
    numbers.push(5);
    numbers.push(5);

    println!("{:?}", numbers);

    numbers.pop();
    numbers.pop();

    println!("{:?}", numbers);

    for x in numbers.iter() {
        println!("{}", x);
    }

    for x in numbers.iter_mut() {
        *x = *x * *x;
    }

    println!("{:?}", numbers);
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
