#![allow(dead_code)]
#![allow(unused_variables)]

// casting is for data types
// conversion is for custome data types

// If you are able to convert type A from type B, then it should be easy
// to convert type B to type A

// From:
// the from trait allows for a type to befine how to create itself from another type,

// Into:
// the into trait is simply the recipical of From.
// it defines how to convert a type into another type.

// From and Into are designed to be complementry.
// we don't need to provide an implementation for both.
// if you implemented the from trait, the into trait will call it when necessary.

pub fn from() {
    // for example we can convert a str to a String
    let my_str = "hello";
    let my_string = String::from(my_str);

    let num = Number::from(30);
    println!("My number is {:?}", num);

    // convert int into a number
    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num);

    println!("from");
    let b1 = BinaryData::from("hello world".to_string());
    println!("{b1:?}");

    let b2: BinaryData = String::from("hi").into();
    println!("{b2:?}");
}

#[derive(Debug)]
struct Number {
    value: i32,
}

// define how to create a Number from a i32
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

// impl Into<Number> for i32 {
//     fn into(self) -> Number {
//         Number { value: self }
//     }
// }

#[derive(Debug)]
struct BinaryData {
    data: Vec<u8>,
}

impl From<String> for BinaryData {
    fn from(input: String) -> Self {
        let data = input.into_bytes();
        BinaryData { data }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_from() {
        super::from();
    }
}
