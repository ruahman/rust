struct Person {
    name: String,
}

// if you car making a refrence to another struct
// make sure you give a life time to the struct and the field that will reference
// the other struct
// that way the compiler will know that the struct that is being referenced
// will live as long as the struct that is referencing it
struct Company<'z> {
    name: String,
    ceo: &'z Person,
}

struct Person2<'a> {
    name: &'a str,
}

// you also need to specify the lifetime of the struct
// if you functions are using that refrece
impl<'a> Person2<'a> {
    fn new(name: &'a str) -> Self {
        Self { name }
    }
    fn talk(&self) {
        println!("Hello, my name is {}", self.name);
    }
}

pub fn run() {
    // lifetime is how long a variable will live
    // a static lifetime is the entire duration of the program
    let s: &'static str = "I have a static lifetime.";
    println!("{}", s);

    let person = Person {
        name: String::from("John"),
    };
    // if property that is refrencing the other sturct
    // does not have a lifetime the compiler will throw an error
    // you need to think in lifetimes when you have a struct that is
    // referencing another struct
    let company = Company {
        name: String::from("Company"),
        ceo: &person,
    };
    println!("{} is the CEO of {}", company.ceo.name, company.name);

    // because the struct has a lifetime the compiler
    // will make sure that the string that is being referenced
    // has the same lifetime as the struct that is referencing it
    // if not the compiler will throw an error
    let person2 = Person2 { name: "John" };
    // lifetime has to be specifed in methods as well
    let persoin3 = Person2::new("John");
    println!("{}", person2.name);
    println!("{}", persoin3.name);
    persoin3.talk();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lifetime() {
        run();
    }
}
