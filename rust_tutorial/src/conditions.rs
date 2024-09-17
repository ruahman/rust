use std::cmp::Ordering;

#[allow(unused_variables)]
#[allow(clippy::manual_range_contains)]
pub fn run() {
    //// if statements
    let age = 42;

    if (age > 1) && (age < 21) {
        println!("You can't drink");
    } else if (age >= 21) && (age < 65) {
        println!("You can drink");
    } else {
        println!("You can't drink anymore");
    }

    //// ternary operators
    #[allow(clippy::needless_bool)]
    let can_vote = if age >= 18 { true } else { false };

    let x = 3;
    #[allow(clippy::needless_bool)]
    let inline_bool = if x < 18 { true } else { false };

    //// match
    match age {
        1..=18 => println!("important years"),
        21 | 30 | 40 => println!("other important years"),
        65.. => println!("retirement"),
        _ => println!("default"),
    }

    //// loops
    let mut x = 1;
    while x < 1000 {
        x *= 2;
        if x == 64 {
            continue;
        }
        println!("while x = {}", x);
    }

    let mut y = 1;
    loop {
        y *= 2;
        if y > 1000 {
            break;
        }
        println!("while y = {}", y);
    }

    for x in 1..11 {
        if x == 3 {
            continue;
        }
        if x == 8 {
            break;
        }
        println!("for x = {}", x);
    }

    for (idx, val) in (30..41).enumerate() {
        println!("idx = {}, val = {}", idx, val);
    }

    //// match
    let country_code = 44;
    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=999 => "unknown",
        _ => "invalid",
    };
    println!("country = {}", country);

    let my_age = 43;
    let voting_age = 18;

    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("can't vote"),
        Ordering::Greater => println!("can vote"),
        Ordering::Equal => println!("can vote"),
    }
}

// cargo test variables::tests -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_conditions() {
        run();
    }
}
