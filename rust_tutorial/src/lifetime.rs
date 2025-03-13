// how long the variable lives

// a type of generic insuring that a reference is valid for as long as needed

// every refrerence has a lifetime, which is the scope for which that reference is valid

// most of the the time this is implicit and inferred by the compiler

// sometimes though, lifetime anoations are needed to help the compiler

// the point of lifetimes is to prevent dangling references,
// which cause a program to reference data that has been deallocated

// the borow checker that copares the scopes/lifetime of refrereces to the data
// they refer to, to ensure that all borrows are valid

// a reference to a value must never outlive the value it references

// a short lifetime can not be coerced into a longer lifetime

// lifetimes of references must outlive the functions that use them

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
    // this will cause a problem
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("r: {}", r);

    // by default this is a staitc lifetime
    #[allow(unused_variables)]
    let x = "foobar";

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

    // most of the time the compiler can infer the lifetimes
    // but sometimes  we have to specify lifetimes to help the borrow checker detrmine
    // if refrences are valid
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("long string is long");
    let result: &str;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    // this can cause a problem because wath if string2 was returned
    // println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lifetime() {
        run();
    }
}
