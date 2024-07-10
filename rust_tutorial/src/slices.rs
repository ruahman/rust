pub fn run() {
    println!("--- slices ---");

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

    let my_string_literal = "hello world";
    let word = &my_string_literal[0..6];
    println!("{}", word);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..4];
    println!("{:?}", slice);

    let animals = ["dog", "cat", "mouse"];
    let anim = &animals[1..3];
    println!("anim: {anim:?}");
}

#[cfg(test)]
mod slices_tests {
    use super::*;
    #[test]
    fn test_slice() {
        run();
    }
}
