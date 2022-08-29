pub fn run() {
  // vec is a mutable array
  let mut numbers: Vec<i32> = vec![1, 2, 3];

  numbers.push(5);
  numbers.push(5);
  numbers.push(5);

  println!("{:?}", numbers);

  numbers.pop();
  numbers.pop();

  println!("{:?}", numbers);

  for x in numbers.iter() {
    println!("{}", x);
  }

  for x in numbers.iter_mut() {
    *x = *x * *x;
  }

  println!("{:?}", numbers);
}
