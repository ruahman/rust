#[allow(dead_code)]
#[derive(Debug)]
struct Conditions {
    less_than_20: bool,
    inline_bool: bool,
    young_man: bool,
}
fn conditions() -> Conditions {
    let less_than_20: bool;
    let x = 18;

    if x < 20 {
        less_than_20 = true
    } else {
        less_than_20 = false
    }

    let inline_bool = if x < 18 { true } else { false };

    let young_man: bool;

    let age: i32 = 41;
    match age {
        1..=39 => young_man = true,
        _ => young_man = false,
    };

    return Conditions {
        less_than_20,
        inline_bool,
        young_man,
    };
}

pub fn demo() {
    println!("conditions: {:?}", conditions())
}

#[cfg(test)]
mod conditions_tests {
    use super::*;
    #[test]
    fn condition_test() {
        let result = conditions();
        assert!(result.less_than_20);
        assert!(result.inline_bool == false);
        assert!(result.young_man == false);
    }
}
