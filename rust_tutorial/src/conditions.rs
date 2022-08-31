pub fn demo() {
  let x = 18;

  if x < 20 {
    println!("less than 20");
  } else {
    println!("more than 20");
  }

  let y = if x < 18 { true } else { false };
  println!("{}", y);

  let age: i32 = 41;
  match age {
      1..=39 => println!("young man"),
      _ => println!("middle age"), 
  };
}
