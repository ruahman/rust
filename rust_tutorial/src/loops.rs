pub fn exec() {
    //// loops

    let arr = [1, 2, 3, 4, 5];
    let mut idx = 0;
    loop {
        if arr[idx] % 2 == 0 {
            println!("even");
        }
        if idx >= arr.len() - 1 {
            break;
        }
        idx = idx + 1;
    }

    //// while

    idx = 0;
    while idx < arr.len() {
        println!("arr: {}", arr[idx]);
        idx = idx + 1;
    }

    //// for

    for item in arr.iter() {
        println!("item: {}", item);
    }

    println!("loops");

    let mut x = 0;

    loop {
        println!("{}", x);
        if x > 10 {
            break;
        }
        x += 1;
    }

    let mut count = 0;

    while count < 100 {
        println!("while: {}", count);
        count += 1;
    }

    for x in 0..100 {
        println!("for: {}", x);
    }
}

// cargo test loops::tests -- --nocapture
#[cfg(test)]
mod tests {
    use super::exec;

    #[test]
    fn test_exec() {
        exec()
    }
}
