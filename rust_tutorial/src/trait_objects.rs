// in rust we define shared behavior using traits.

// static and dynamic dispatch
// static dispatch is when the compiler knows which method to call at compile time
// dynamic dispatch is when the compiler doesn't know which method to call at compile time
// the dyn keyword is used to indicate that we are using dynamic dispatch

pub fn run() {
    trait Draw {
        fn draw(&self);
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Button {
        width: u32,
        height: u32,
        label: String,
    }

    impl Draw for Button {
        fn draw(&self) {
            println!("Drawing a button {self:?}:");
        }
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }

    impl Draw for SelectBox {
        fn draw(&self) {
            println!("Drawing a select box {self:?}:");
        }
    }

    // if all the types are homogoeneous, we can use generics instead of trait objects
    // however, if the itemes in the vector are of different types, we can use trait objects
    struct Screen {
        components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_trait_objects() {
        run();
    }
}
