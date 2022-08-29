pub fn demo(){
    let mut hello = String::from("Hello form strings");
    println!("{}", hello);
    hello.push_str(" push me");
    println!("{}", hello);

    for word in hello.split_whitespace() {
        println!("{}", word);
    }
}
