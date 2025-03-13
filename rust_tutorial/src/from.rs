#![allow(dead_code)]

pub fn run() {
    println!("from");
    let b1 = BinaryData::from("hello world".to_string());
    println!("{b1:?}");

    let b2: BinaryData = String::from("hi").into();
    println!("{b2:?}");
}

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
        super::run();
    }
}
