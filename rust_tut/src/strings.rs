#![allow(dead_code)]
#![allow(unused_mut)]

// a str is just a slice of a String

// a String has a pointer,
// length,
// and capacity: how much more data can be added before it needs to be realocated somewhere else
// a str is just a pointer and length

pub fn strings() {
    //// strings

    // String is heep allocated, owns the data
    // &str is a string slice, it is a refrence to a string, it does not own the data

    // string literals are (&'static str), meaning that they are stored in the static memory
    // and are available for the entire duration of the program

    // String - owned
    // * used to store string in a struct
    // &str - borrowed string slice
    // * used to pass string slices to functions

    // strings come in two forms
    // 1. string literals
    // 2. string objects
    // string literals are immutable
    // string objects are mutable
    // string literals are stored in the static memory
    // string objects are stored in the heap memory
    // string literals are faster than string objects
    // string literals are not mutable
    // string objects are mutable

    // string literals, this is a refrence to a string slice
    // this is alocated at compile time
    let str1 = "Hello there!";
    assert!(str1 == "Hello there!");

    // &str is a string slice, this is a slice out of a string
    // string slices can not be modified
    // 'static is a lifetime specifier
    // 'static means that the string is stored in the static memory
    // this is just a string slice from the static memory, it comes with the program
    let str2: &'static str = "Hello there!";
    assert!(str2 == "Hello there!");

    // cant do this with string slices
    // println!("{}", str2[0]);

    // but you can do this with string slices
    for char in str2.chars() {
        println!("{}", char);
    }

    // and this
    if let Some(first_char) = str2.chars().next() {
        assert!(first_char == 'H');
    }

    // string objects
    // string objects are mutable
    // string objects are stored in the heap memory
    // can be created at run time.
    let mut str1 = String::new();
    str1.push('A');
    str1.push_str(" hello");
    str1.push_str(" world");
    assert!(str1 == "A hello world");

    let str_slice: &str = &str1;
    assert!(str_slice == "A hello world");

    for word in str1.split_whitespace() {
        println!("{}", word);
    }

    let str2 = str1.replace("A", "the");
    assert!(str2 == "the hello world");

    // create string from string literal???
    let str3 = String::from("x r t b h k k a m c");
    let mut v1: Vec<char> = str3.chars().collect();
    v1.sort();
    v1.dedup(); // remove duplicates
    for char in v1 {
        print!("{}", char);
    }

    // string literal
    let str4: &str = "Random string";
    // convert string literal to string object
    let mut str5: String = str4.to_string();
    assert!(str4 == str5);

    // create string slice from string
    let str6 = &str5[0..6];
    assert!(str6 == "Random");

    let mut hello = String::from("Hello form strings");
    assert!(hello == "Hello form strings");
    hello.push_str(" push me");
    assert!(hello == "Hello form strings push me");

    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // string literals
    // are just string slices
    let greeting: &str = "Nice to meet you";
    assert!(greeting == "Nice to meet you");

    // this string slice in alocated in the static memory
    let s: &'static str = "hello there!";
    assert!(s == "hello there!");

    // format
    let name = "Diego";
    let greeting = format!("Hi, I'm {}", name);
    assert!(greeting == "Hi, I'm Diego");
}

// cargo test strings::tests -- --nocapture
#[cfg(test)]
mod tests {
    use super::strings;

    #[test]
    fn test_strings() {
        strings()
    }
}
