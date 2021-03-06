pub fn run() {
  println!("functions");
  greetings("hi", "diego");
  println!("{}", add(2, 2));

  // closure
  let n3: i32 = 10;
  let add_num = |n1: i32, n2: i32| n1 + n2 + n3;
  println!("{}", add_num(1, 2));
}

fn greetings(greet: &str, name: &str) {
  println!("{}:{}", greet, name);
}

fn add(x: i32, y: i32) -> i32 {
  x + y
}
