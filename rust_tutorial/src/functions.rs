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
#[allow(dead_code)]
fn never_returns() -> ! {
    // use this macro or todo!() to indicate that the function is not implemented
    // yet to the compiler.
    unimplemented!();
}

#[allow(dead_code)]
pub fn run() {
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
    let add_num = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("{}", add_num(1, 2));

    let mut z = 5;
    increase(&mut z);
    println!("z: {}", z);
}

fn greetings(greet: &str, name: &str) {
    println!("{}:{}", greet, name);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

// cargo test variables::tests -- --nocapture
#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_functions() {
        run()
    }
}
