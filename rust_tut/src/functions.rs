#![allow(dead_code)]

fn increase(x: &mut i32) {
    *x += 1;
}

#[allow(dead_code)]
fn say_hello() {
    println!("hello");
}

fn get_sum(x: i32, y: i32) -> i32 {
    x + y
}

fn sum_list(list: &[i32]) -> i32 {
    let mut sum = 0;
    for x in list.iter() {
        sum += x;
    }
    sum
}

// divergent function:
// a function that never returns.
// This is indicated using the special return type !,
// which is an empty type that has no values.
fn never_returns() -> ! {
    // use this macro or todo!() to indicate that the function is not implemented
    // yet to the compiler.
    unimplemented!();
}

fn greetings(greet: &str, name: &str) {
    println!("{}:{}", greet, name);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

// a funtion that returns a function
fn greater_than(limit: u32) -> impl Fn(u32) -> bool {
    // clojure take ownership
    move |y| y > limit
}

pub fn functions() {
    //// functions

    say_hello();
    let res = get_sum(1, 2);
    println!("res: {}", res);

    let list = vec![1, 2, 3, 4, 5];
    println!("sum: {}", sum_list(&list));

    println!("functions");
    greetings("hi", "diego");
    println!("{}", add(2, 2));

    // ref mut
    let mut z = 1;
    increase(&mut z);
    println!("{}", z);

    // closure
    let n3: i32 = 10;
    let add_num = |n1: i32, n2: i32| n1 + n2 + n3; // n3 is just a reference
    println!("{}", add_num(1, 2));
    println!("n3: {}", n3);

    let my_string = "foobar".to_string();
    // closure is just a ref
    let show_string = || println!("{my_string}");
    show_string();
    println!("{my_string}");

    // clojure takes ownership now
    let show_string_move = move || println!("{my_string}");
    show_string_move();
    // println!("{my_string}"); // can't do this now

    let mut z = 5;
    increase(&mut z);
    println!("z: {}", z);

    let greater_than_9 = greater_than(9);
    println!("{}", greater_than_9(10));
}

// cargo test variables::tests -- --nocapture
#[cfg(test)]
mod tests {
    use super::functions;

    #[test]
    fn test_functions() {
        functions()
    }
}
