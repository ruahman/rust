#[allow(unused_variables)]
#[allow(dead_code)]
pub fn run() {
    //// vector

    // a vector is something that grows dynamically
    // vector must be mutable to push
    // basically a dynamic array
    let mut vec2 = vec![1, 2, 3];
    vec2.push(4);
    println!("fst: {}", vec2[0]);

    // get returns an option type
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

    // show debug
    println!("debug: {numbers:?}");

    let pop = numbers.pop();
    if let Some(x) = pop {
        println!("Popped: {}", x);
    }

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
    use super::run;

    #[test]
    fn test_vectors() {
        run()
    }
}
