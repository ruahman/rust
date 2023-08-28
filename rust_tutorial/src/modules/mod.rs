mod test {
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

mod pizza_order {
    pub struct Pizza {
        pub dough: String,
        pub cheese: String,
        pub topping: String,
    }
    impl Pizza {
        pub fn lunch(tooping: &str) -> Pizza {
            Pizza {
                dough: String::from(tooping),
                cheese: String::from(tooping),
                topping: String::from(tooping),
            }
        }
    }
    pub mod help_customer {
        pub fn seat_at_table() {
            println!("seat at table");
        }
        #[allow(dead_code)]
        pub fn take_order() {
            seat_at_table();
            println!("take order");
        }
    }
}

#[allow(unused_variables)]
#[allow(dead_code)]
pub fn exec() {
    let x = crate::modules::test::Pizza::lunch("test");
    let y = crate::modules::pizza_order::Pizza::lunch("test");
    println!("hello from modules");
    println!("{:?}", x);
}

// cargo test variables::tests -- --nocapture
#[cfg(test)]
mod tests {
    use super::exec;

    #[test]
    fn test_exec() {
        exec()
    }
}
