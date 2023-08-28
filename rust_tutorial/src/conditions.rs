use std::cmp::Ordering;

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_assignments)]
pub fn exec() {
    //// if statements
    let age = 42;
    if (age > 1) && (age < 21) {
        println!("You can't drink");
    } else {
        println!("You can drink");
    }

    let less_than_20: bool;
    let x = 18;

    if x < 20 {
        less_than_20 = true
    } else {
        less_than_20 = false
    }

    //// ternary operators
    let can_vote = if age >= 18 { true } else { false };

    let inline_bool = if x < 18 { true } else { false };

    //// match
    match age {
        1..=18 => println!("important years"),
        21 | 30 | 40 => println!("other important years"),
        65.. => println!("retirement"),
        _ => println!("default"),
    }

    let my_age = 43;
    let voting_age = 18;

    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("can't vote"),
        Ordering::Greater => println!("can vote"),
        Ordering::Equal => println!("can vote"),
    }

    let young_man: bool;

    let age: i32 = 41;
    match age {
        1..=39 => young_man = true,
        _ => young_man = false,
    };
}

// cargo test variables::tests -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn condition_test() {
        exec();
    }
}
