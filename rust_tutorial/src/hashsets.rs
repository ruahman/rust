use std::collections::HashSet;

pub fn run() {
    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");

    // nothing will happen
    greeks.insert("delta");

    println!("{:?}", greeks);

    let added_vega = greeks.insert("vega");
    // if we managed to add an object
    if added_vega {
        println!("We added vega");
    }

    if !greeks.contains("kappa") {
        println!("We don't have kappa");
    }

    let removed = greeks.remove("delta");
    if removed {
        println!("We removed delta");
    }

    println!("{:?}", greeks);

    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();
    let _1_10: HashSet<_> = (1..=10).collect();

    println!(
        "Is {:?} a subset of {:?}? {}",
        _2_8,
        _1_10,
        _2_8.is_subset(&_1_10)
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hashsets() {
        run();
    }
}
