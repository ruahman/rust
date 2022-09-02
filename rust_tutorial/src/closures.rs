fn use_func<T>(a: i32, b: i32, func: T) -> i32
where
    T: Fn(i32, i32) -> i32,
{
    func(a, b)
}

pub fn demo() {
    let can_vote = |age: i32| age >= 18;
    println!("{}", can_vote(42));
    let sum = |a, b| a + b;
    println!("{}", use_func(5, 4, sum));

    let plus_one = |x: i32| -> i32 { x + 1 };
    let x = plus_one(2);
    println!("{}", x);

    let above_limit = greater_than(100);
    println!("{}", above_limit(10));
}

fn greater_than(limit: u32) -> impl Fn(u32) -> bool {
    // lifetime of limit stays with clojure
    move |x| x > limit
}
