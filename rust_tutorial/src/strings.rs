#[allow(dead_code)]
#[allow(unused_mut)]
pub fn run() {
    //// strings

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

    // string literals
    let str1 = "Hello there!";

    // &str is a string slice, this is a slice out of a string
    // string slices can not be modified
    // 'static is a lifetime specifier
    // 'static means that the string is stored in the static memory
    // this is just a string slice from the static memory, it comes with the program
    let str2: &'static str = "Hello there!";

    // cant do this with string slices
    // println!("{}", str2[0]);

    // but you can do this with string slices
    for char in str2.chars() {
        println!("{}", char);
    }

    // and this
    if let Some(first_char) = str2.chars().nth(0) {
        println!("First char: {}", first_char);
    }

    println!("{},{}", str1, str2);

    // string objects
    // string objects are mutable
    // string objects are stored in the heap memory
    let mut str1 = String::new();
    str1.push('A');
    str1.push_str(" hello");
    str1.push_str(" world");

    let str_slice: &str = &str1;
    println!("{}", str_slice);

    for word in str1.split_whitespace() {
        println!("{}", word);
    }

    let str2 = str1.replace("A", "the");
    println!("{}", str2);

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
    let mut str5: String = str4.to_string();
    println!("{}:{}", str4, str5);

    // create string slice from string
    let str6 = &str5[0..6];
    println!("{}", str6);

    let mut hello = String::from("Hello form strings");
    println!("{}", hello);
    hello.push_str(" push me");
    println!("{}", hello);

    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // string literals
    // are just string slices
    let greeting: &str = "Nice to meet you";
    println!("{}", greeting);

    // this string slice in alocated in the static memory
    let s: &'static str = "hello there!";
    println!("{}", s);
}

// cargo test strings::tests -- --nocapture
#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_strings() {
        run()
    }
}
