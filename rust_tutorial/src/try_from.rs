#![allow(dead_code)]

// incase of an error in conversion use TryFrom
// since there is no try catch in rust

use std::convert::{TryFrom, TryInto};

pub fn run() {
    println!("try from");
    let t1 = (String::from("Jim"), 20);
    let t2 = (String::from("Joe"), 30);
    let p1 = Person::try_from(t1);
    let p2: Result<Person, String> = t2.try_into();
    println!("{p1:?},{p2:?}");

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));
}

#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

impl TryFrom<(String, i32)> for Person {
    type Error = String;
    fn try_from(value: (String, i32)) -> Result<Self, Self::Error> {
        if value.1 > 100 {
            Err("age too old".to_string())
        } else {
            Ok(Person {
                name: value.0,
                age: value.1,
            })
        }
    }
}

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_try_from() {
        run();
    }
}
