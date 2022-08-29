pub fn run() {
  let x = 18;

  if x < 20 {
    println!("less than 20");
  } else {
    println!("more than 20");
  }

  let y = if x < 18 { true } else { false };
  println!("{}", y);
}
