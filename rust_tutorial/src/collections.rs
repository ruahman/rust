use std::collections::{HashMap, HashSet};

pub fn demo() {
    // vectors
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    a.push(4);
    println!("{:?}", a);
    println!("{}", a[2]);

    if let Some(x) = a.get(2) {
        println!("{}", x);
    }

    while let Some(x) = a.pop() {
        println!("{}", x);
    }

    // Hashmap
    let mut shapes = HashMap::new();
    shapes.insert(String::from("xxx"), 34);
    shapes.insert(String::from("yyy"), 31);
    shapes.insert(String::from("zzz"), 32);
    shapes.insert("eee".into(), 44);

    for (key, val) in &shapes {
        println!("{}{}", key, val);
    }

    //HashSet
    let mut g = HashSet::new();
    g.insert("gamma");
    g.insert("delta");
    g.insert("delta");
    if g.contains("delta") {
        println!("I have the delta");
    }

    // iterators
    let vec = vec![3, 2, 1];

    // use & so that we dont move the value
    for x in &vec {
        // x is a ref
        println!("{}", *x);
    }

    // this is better
    for x in vec.iter() {
        println!("{}", x);
    }
}
