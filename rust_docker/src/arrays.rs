use std::mem;

pub fn run() {
  let numbers: [i32; 5] = [1, 2, 3, 4, 5];

  // debug trait
  println!("{:?}", numbers);
  println!("{}", numbers.len());
  println!("{}", mem::size_of_val(&numbers));

  let slice: &[i32] = &numbers[1..3];
  println!("{:?}", slice);
  println!("{:?}", &numbers[0..3]);
  // println!("{:?}", numbers[0..3]); // causes problems
}
