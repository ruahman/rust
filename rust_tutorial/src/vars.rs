pub fn demo() {
  let my_name = "diego";
  let mut age = 37;
  println!("{}", age);
  age = 40;
  println!("{}", age);

  println!("my name is {}", my_name);

  const ID: i32 = 001;
  println!("ID: {}",ID);

  let (my_namex, my_age) = ("Brad", 37);
  println!("{}{}",my_namex,my_age);
}
