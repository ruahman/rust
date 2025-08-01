#![allow(dead_code)]

use std::collections::HashSet;

// this is inserted into the code
// - macro are switch statements that run at compile time
macro_rules! say_hello {
    () => {
        println!("Hello!")
    };
}

// ty is type, expr is expression
// macro_rules! bounded_imp {
//     ($t:ty, $min:expr, $max:expr) => {
//         impl Bounded for $t {
//             #[inline]
//             fn min_value() -> $t {
//                 $min
//             }
//             #[inline]
//             fn max_value() -> $t {
//                 $max
//             }
//         }
//     };
// }

// there are two types of macros.
// decalaritive macros that just do a simple syntax rewrite and
// procedure macros that can do a sysntax rewrite and execute code at compile time

// macros are boiler plate killers

// macros take a fragment of any code as input and produce rustcode as output

// procedure macros allow you to make anotations
// #[some_attribute]
// pub fn some_name(....) {
//
// }

// prodedure macros also allow you to make custome derive macros
// #[derive(HelloMacro)]
// struct Pancakes;
//
// Pancakes::hello_macor();

// macros are like match clauses

macro_rules! nothing_burger {
    () => {};
}

macro_rules! hello_ferris {
    () => {
        println!("Where's ferris");
    };
    ("Ferris") => {
        println!("Hello Ferris");
    };
}

// if we want to match on a arbitry literal instead of "Ferris"
// we use something called captures
// a capture is like a variable but it's type is a code fragment type
macro_rules! hello_x {
    // specific literal
    ("Bob") => {
        println!("Hello, Bob");
    };
    // literal
    ($s:literal) => {
        println!("Hello, {}", $s)
    };
    // variable identifier
    ($s:ident) => {
        println!("Hello, {}", $s)
    };
    // expression covers literal, identefiers and expressions
    ($s:expr) => {
        println!("express: {}", $s);
    };
    ($( $s:expr ),+) => {
        $(
            println!("repeat: {}", $s);
        )*
    }
}

// repetitions allow you to capture any number of code fragments
macro_rules! set {
    // specify the syntax type, seperator and repeater
    ($($s:expr),*) => {
        // specify how you like to extact the repeater
        HashSet::from([$($s),*])
    };
}

macro_rules! my_vec {
    // repeatition
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            // expand repetion values
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

pub fn macros() {
    say_hello!();
    nothing_burger!();
    hello_ferris!();
    hello_ferris!("Ferris");
    hello_x!("Diego");
    let a = "andy";
    hello_x!(a);
    hello_x!("Bob");
    hello_x!(1 + 3);
    hello_x!("Alice", "Bob", "Carol");
    let b = set!(1, 2, 3);
    dbg!(b);
    let c = my_vec!(4, 5, 6);
    dbg!(c);

    // bounded_imp!(u8, u8::MIN, u8::MAX);
    // this creates
    // impl Bounded for u8 {
    //      #[inline]
    //      fn min_value() -> u8 {
    //          u8::MIN
    //      }
    //      #[inline]
    //      fn max_value() -> u8 {
    //          u8:MAX
    //      }
    // }
}

#[cfg(test)]
mod tests {
    use super::macros;

    #[test]
    fn test_macros() {
        macros();
    }
}
