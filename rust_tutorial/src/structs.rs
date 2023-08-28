struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple struct
struct TupColor(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string()
    }
}

#[allow(dead_code)]
struct Customer {
    name: String,
    address: String,
    balance: f32,
}

#[allow(dead_code)]
struct Rectangle<T, U> {
    width: T,
    height: U,
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn exec() {
    let mut bob = Customer {
        name: String::from("bob"),
        address: String::from("123 main"),
        balance: 0.0,
    };
    bob.address = String::from("123 main st");

    let rec = Rectangle {
        width: 10,
        height: 20.5,
    };

    let c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    println!("{}{}{}", c.red, c.green, c.blue);

    let x = TupColor(1, 2, 3);

    println!("{}{}{}", x.0, x.1, x.2);

    let mut p = Person::new("diego", "vila");

    println!("Person {}{}", p.first_name, p.last_name);
    println!("{}", p.full_name());
    p.set_last_name("bennet");
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
