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
}
