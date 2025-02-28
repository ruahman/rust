use std::mem;

static X: [i32; 5] = [1, 2, 3, 4, 5];

// This function borrows a slice.
fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

#[allow(clippy::needless_range_loop)]
pub fn run() {
    // arrays are fixed size
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a len: {}", a.len());

    // index and element
    a[0] = 123;
    println!("a[0]: {}", a[0]);

    // print the whole array
    println!("a: {:?}", a);

    // loop through array
    for i in 0..a.len() {
        println!("a[{}]: {}", i, a[i]);
    }

    // array of an array
    let mtx: [[f32; 3]; 2] = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]];

    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            println!("mtx[{}][{}]: {}", i, j, mtx[i][j]);
        }
    }

    //// arrays
    let arrys: [i32; 5] = [1, 2, 3, 4, 5];
    println!("arrys: {:?}", arrys);
    println!("item 1: {}", arrys[0]);
    println!("arrys len: {}", arrys.len());

    // arrays have a fixed lenght
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("numbers: {:?}", numbers);

    // slices point to a slice of memory of a array
    let slice: &[i32] = &X[1..3];
    println!("slice: {:?}", slice);

    // this doesn't work, must be reference
    // let slice: &[i32] = X[1..3]

    // Fixed-size array
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value.
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0.
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    // `len` returns the count of elements in the array.
    println!("Number of elements in array: {}", xs.len());

    // Arrays are stack allocated.
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices.
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    // Slices can point to a section of an array.
    // They are of the form [starting_index..ending_index].
    // `starting_index` is the first position in the slice.
    // `ending_index` is one more than the last position in the slice.
    println!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[1..4]);

    // Arrays can be safely accessed using `.get`, which returns an
    // `Option`. This can be matched as shown below, or used with
    // `.expect()` if you would like the program to exit with a nice
    // message instead of happily continue.
    for i in 0..xs.len() + 1 {
        // Oops, one element too far!
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

    // return Arrays { numbers, slice };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arrays() {
        run();
    }
}
