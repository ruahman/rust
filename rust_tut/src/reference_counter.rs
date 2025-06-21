use std::rc::Rc;

// sometimes you want to have multiple ownership of a value
// Rc is a reference counter that allows you to have multiple ownership of a value

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

pub fn reference_counter() {
    let name = Rc::new("John".to_string());
    println!("name count: {}", Rc::strong_count(&name));
    {
        // instead of clone the string it just makes another refrence
        let person = Person::new(name.clone());
        person.greet();
        println!("name count: {}", Rc::strong_count(&name));
        let another_referenct = name.clone();
        println!("another_reference: {}", another_referenct);
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
        reference_counter();
    }
}
