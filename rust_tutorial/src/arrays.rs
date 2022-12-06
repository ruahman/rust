static X: [i32; 5] = [1, 2, 3, 4, 5];

#[allow(dead_code)]
#[derive(Debug)]
struct Arrays {
    numbers: [i32; 5],
    slice: &'static [i32],
}
fn arrays<'a>() -> Arrays {
    // arrays have a fixed lenght
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // slices point to a slice of memory of a array
    let slice: &[i32] = &X[1..3];

    // this doesn't work
    // let slice: &[i32] = X[1..3]

    return Arrays { numbers, slice };
}

#[allow(unused_variables)]
pub fn demo() {
    println!("--- arrays ---");

    println!("arrays: {:?}", arrays());
}

#[cfg(test)]
mod arrays_tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_arrays() {
        let res = arrays();
        assert_eq!(res.numbers, [1, 2, 3, 4, 5]);
        assert_eq!(res.numbers.len(), 5);
        assert_eq!(mem::size_of_val(&res.numbers), 20);
        assert_eq!(res.slice.len(), 2)
    }
}
