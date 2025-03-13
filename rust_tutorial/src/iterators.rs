pub fn run() {
    let vec = vec![1, 2, 3, 4, 5];

    // don't do this it will move vec and you cant use it again???
    // for x in vec {
    //     println!("{}", x);
    // }

    // this is better
    for x in vec.iter() {
        println!("{}", x);
    }

    // reverse
    for x in vec.iter().rev() {
        println!("{}", x);
    }

    println!("vec: {:?}", vec);

    let mut vec2 = vec![1, 2, 3, 4, 5];
    vec2.extend(vec);

    // you move element from vec to vec2,
    // so you can't use vec again
    println!("vec2: {:?}", vec2);

    // this will cause problems
    // println!("vec: {:?}", vec);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_iterators() {
        run();
    }
}
