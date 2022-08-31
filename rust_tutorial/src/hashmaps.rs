use std::collections::HashMap;

pub fn demo() {
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
