#[allow(dead_code)]
#[allow(unused_mut)]
pub fn exec() {
    //// strings

    let mut str1 = String::new();
    str1.push('A');
    str1.push_str(" hello");
    str1.push_str(" world");

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
    use super::exec;

    #[test]
    fn test_exec() {
        exec()
    }
}
