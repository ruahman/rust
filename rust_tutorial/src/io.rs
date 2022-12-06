use std::io;

pub fn stdio() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
}
pub fn demo() {
    println!("***** io demo *****");

    println!("Please enter something");
    let res = stdio();

    println!("You entered: {res}");
}
