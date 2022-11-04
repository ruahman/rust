use std::mem;

#[allow(unused_variables)]
pub fn demo() {
    // arrays have a fixed lenght
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // debug trait
    println!("{:?}", numbers);
    println!("{}", numbers.len());
    println!("{}", mem::size_of_val(&numbers));

    // slices point to memory of array
    let slice: &[i32] = &numbers[1..3];
    println!("{:?}", slice);
    println!("{:?}", &numbers[0..3]);
    // println!("{:?}", numbers[0..3]); // causes problems

    // array of string literals
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // specify size of array and type
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // initialize array with 5
    let a = [3; 5];

    // access individual elements
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
