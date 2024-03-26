use std::rc::Rc;

struct Person {
    name: Rc<String>,
}

impl Person {
    fn new(name: Rc<String>) -> Self {
        Self { name }
    }
    fn greet(&self) {
        println!("Hello, my name is {}", self.name);
    }
}

pub fn run() {
    let name = Rc::new("John".to_string());
    println!("name count: {}", Rc::strong_count(&name));
    {
        let person = Person::new(name.clone());
        person.greet();
        println!("name count: {}", Rc::strong_count(&name));
    }
    println!("name count: {}", Rc::strong_count(&name));
    // if name was not a reference counter then this would not work
    println!("{}", name);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reference_counter() {
        run();
    }
}
