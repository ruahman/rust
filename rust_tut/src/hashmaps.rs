use std::collections::HashMap;

#[allow(dead_code)]
pub fn hashmap() {
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);
    // force string literal to be a String
    shapes.insert("pentagon".into(), 5);

    // convert "square" to a proper String
    println!("A square has {} sides", shapes["square"]);
    println!("shapes: {:?}", shapes);

    let mut heroes: HashMap<&str, &str> = HashMap::new();
    heroes.insert("superman", "clark");
    heroes.insert("batman", "bruse");
    heroes.insert("spiderman", "peer");

    for (k, v) in heroes.iter() {
        println!("{}{}", k, v);
    }

    if heroes.contains_key(&"batman") {
        let batman = heroes.get(&"batman");
        match batman {
            Some(x) => println!("{}", x),
            None => println!("nothing"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::hashmap;

    #[test]
    fn test_hashmap() {
        hashmap()
    }
}
