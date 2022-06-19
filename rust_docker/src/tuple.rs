pub fn run() {
  let person: (&str, &str, i8) = ("diego", "vila", 40);
  println!("{}:{}:{}", person.0, person.1, person.2);
}
