use std::collections::HashMap;

#[allow(dead_code)]
pub fn exec() {
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

// cargo test hashmaps::tests -- --nocapture
#[cfg(test)]
mod tests {
    use super::exec;

    #[test]
    fn test_exec() {
        exec()
    }
}
