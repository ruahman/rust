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

pub fn demo() {
    let x = crate::modules::test::Pizza::lunch("test");
    println!("hello from modules");
    println!("{:?}", x);
}
