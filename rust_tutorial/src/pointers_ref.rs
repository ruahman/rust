pub fn run() {
  println!("pointers");

  // if you assign a non primative to another variable,
  // you can no longer use the old one.
  // i think that's to prevent errors??
  let x = vec![1, 2, 3];
  let y = &x;
  println!("{:?}", (&x, y));
}
