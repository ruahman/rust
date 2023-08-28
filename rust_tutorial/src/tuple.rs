#[allow(dead_code)]
#[allow(unused_variables)]
pub fn exec() {
    //// tuple

    let my_tuple: (u8, String, f64) = (42, String::from("diego"), 3.14);
    println!("my_tuple: {:?}", my_tuple);
    println!("my_tuple: {}, {}, {}", my_tuple.0, my_tuple.1, my_tuple.2);

    let (age, name, pi) = my_tuple;
    println!("age: {}, name: {}, pi: {}", age, name, pi);

    let person: (&str, &str, i8) = ("diego", "vila", 41);
    println!("{}:{}:{}", person.0, person.1, person.2);

    //// deconstuct tuples
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    // A tuple with a bunch of different types.
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    // Values can be extracted from the tuple using tuple indexing.
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);

    // Tuples can be tuple members.
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable.
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("Pair is {:?}", pair);

    println!("Uhe reversed pair is {:?}", reverse(pair));

    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses.
    println!("One element tuple: {:?}", (5u32,));
    println!("Just an integer: {:?}", (5u32));

    // Tuples can be destructured to create bindings.
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);

    // return Tuples { person, x, y, z };
}

// Tuples can be used as function arguments and as return values.
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables.
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

// The following struct is for the activity.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exec() {
        exec();
    }
}
