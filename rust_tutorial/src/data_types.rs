#[allow(unused_variables)]
pub fn run() {
    // rust has 4 scalar types

    // integers
    let x8: i8 = 23;
    let xu8: u8 = 23;
    let x16: i16 = 230;
    let xu16: u16 = 230;
    let x32: i32 = 2300;
    let xu32: u32 = 2300;
    let x64: i64 = 23000;
    let xu64: u64 = 23000;
    let xs: isize = 230000;

    println!("x8: {}", x8);
    println!("xu8: {}", xu8);

    // integer literals
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';

    // floats
    let y32: f32 = 2.3;
    let y64: f64 = 2.3;

    println!("y32: {}", y32);
    println!("y64: {}", y64);

    // booleans
    let z: bool = true;
    println!("z: {}", z);

    // characters
    let a: char = 'a';
    println!("a: {}", a);

    //// cast

    let int_u8 = 5;
    let int2_u8 = 4;
    let int3_u32 = (int_u8 as u32) + (int2_u8 as u32);
    println!("int3_u32: {}", int3_u32);
}

// cargo test variables::tests -- --nocapture
#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_data_types() {
        run()
    }
}
