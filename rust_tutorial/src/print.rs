pub fn demo() {
  println!("hello print");
  println!("num: {}", 1);
  println!("named param: {param}", param = "foobar");
  // print object that have debug trait implemented
  println!("tuple {:?}", (true, 12, "test"));
}
