// define your modules here
mod pizza_order;
mod pizza_tester;
mod pizza_inline {
    #[derive(Debug)]
    pub struct Pizza {
        pub dough: String,
    }
    impl Pizza {
        pub fn lunch(tooping: &str) -> Pizza {
            Pizza {
                dough: String::from(tooping),
            }
        }
    }
}

pub fn run() {
    // use your modules here
    use pizza_inline::Pizza as pizza_inline;
    use pizza_order::Pizza;
    use pizza_tester::Pizza as pizza_tester;
    let x = pizza_tester::lunch("pizza tester");
    let y = Pizza::lunch("test");
    let z = pizza_inline::lunch("inline");
    println!("hello from modules");
    println!("{:?}", x);
    println!("{:?}", y);
    println!("{:?}", z);
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_modules() {
        run()
    }
}
