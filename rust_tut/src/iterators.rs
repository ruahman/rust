#![allow(dead_code)]

// an iterator is just a object the iterates through a collection
pub fn iterators() {
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

    let foo = vec![1, 2, 3];
    // map just creates an iterator
    // collect launches the iterator
    // iter does not consume your collection
    let foo2: Vec<_> = foo.iter().map(|x| x + 1).collect();

    dbg!(&foo2);

    // into_iter consumes your collection
    let foo3: Vec<_> = vec![1, 2, 3].into_iter().map(|x| x * x).collect();
    dbg!(&foo3);

    let foo4: Vec<_> = vec![1, 2, 3]
        .iter()
        .enumerate()
        .map(|(i, v)| i * v)
        .collect();

    dbg!(&foo4);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_iterators() {
        iterators();
    }
}
