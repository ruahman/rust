pub fn demo() {
  println!("loops");

  let mut x = 0;

  loop {
    println!("{}", x);
    if x > 10 {
      break;
    }
    x += 1;
  }

  let mut count = 0;

  while count < 100 {
    println!("while: {}", count);
    count += 1;
  }

  for x in 0..100 {
    println!("for: {}", x);
  }
}
