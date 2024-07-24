// a closure is an anonymous function that can capture values from the scope
// in which it is defined

// they can take ownership of values by using the move keyword

fn use_func<T>(a: i32, b: i32, func: T) -> i32
where
    T: Fn(i32, i32) -> i32,
{
    func(a, b)
}

pub fn run() {
    let can_vote = |age: i32| age >= 18;
    println!("{}", can_vote(42));
    let sum = |a, b| a + b;
    println!("{}", use_func(5, 4, sum));

    let plus_one = |x: i32| -> i32 { x + 1 };
    let x = plus_one(2);
    println!("{}", x);

    let above_limit = greater_than(100);
    println!("{}", above_limit(10));

    let plus_one = |x: i32| -> i32 { x + 1 };
    let a = 6;
    println!("{0} + 1 = {1}", a, plus_one(a));

    let mut two = 6;
    let plus_two = |x: i32| {
        let mut result = x;
        // two is borrowed here
        result += two;
        result
    };
    println!("{}", plus_two(3));

    let borrow_two = &mut two;
    println!("borrow_two {}", borrow_two);

    let color = String::from("green");
    // color is moved here, so closue takes ownership of color now
    let print = move || println!("{}", color);
    print();
}

fn greater_than(limit: u32) -> impl Fn(u32) -> bool {
    // lifetime of limit stays with clojure
    // that way limit is not dropped
    // this is when you create a function generator, a function that returns a function
    move |x| x > limit
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_clojures() {
        run();
    }
}
