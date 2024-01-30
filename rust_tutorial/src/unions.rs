#![allow(dead_code)]

// Unions are like structs but they can only hold one value at a time.
// list all possible types a union can hold
// but only one value can be held at a time, so you don't know which type is in use
union IntOrFloat {
    i: i32,
    f: f32,
}

fn process_value(iof: IntOrFloat) {
    unsafe {
        match iof {
            IntOrFloat { i: 42 } => {
                println!("life begins at 42");
            }
            IntOrFloat { f } => {
                println!("float value = {}", f);
            }
        }
    }
}

pub fn run() {
    let mut iof = IntOrFloat { i: 123 };
    iof.i = 234;

    // you don't know which type is in use,
    // so you have to use unsafe
    // since this might be empty since it can be either an int or a float
    let value = unsafe { iof.i };
    println!("iof.i = {}", value);

    process_value(IntOrFloat { i: 42 });
    process_value(IntOrFloat { f: 1.23 });
    process_value(IntOrFloat { i: 5 });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unions() {
        run();
    }
}
