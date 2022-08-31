use std::ops::Add;

fn get_sum_gen<T:Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

pub fn demo(){
    println!("{}",get_sum_gen(2,2))
}
