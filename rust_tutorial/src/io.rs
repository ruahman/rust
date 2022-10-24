use std::io;

pub fn demo() {
    println!("***** io demo *****");

    println!("Please enter something");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("You entered: {input}");
}
