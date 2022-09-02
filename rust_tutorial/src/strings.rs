pub fn demo() {
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
