#[allow(unused_variables)]
#[allow(dead_code)]
pub fn vectors() {
    //// vector

    // a vector is something that grows dynamically
    // vector must be mutable to push
    // basically a dynamic array
    let mut vec2 = vec![1, 2, 3];
    vec2.push(4);
    println!("fst: {}", vec2[0]);
    println!("len: {}", vec2.len());

    // get returns an option type
    match vec2.get(1) {
        Some(x) => println!("snd: {}", x),
        None => println!("none"),
    }

    // if you don't borrow then the values are moved and you cant use them anymore
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

    while let Some(x) = numbers.pop() {
        println!("pop {}", x);
    }
}

// cargo test variables::tests -- --nocapture
#[cfg(test)]
mod tests {
    use super::vectors;

    #[test]
    fn test_vectors() {
        vectors()
    }
}
