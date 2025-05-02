#![allow(dead_code)]
#![allow(unused_variables)]

// a lifetime is when a value is allocated and deallocated

// when ever you have references in a struct you need to specify lifetiems

// lifetime elision,  sometime you can ommit specifying a lifetime, rust can figure it out.

// rust allows only one owener of memory but you are allowed to have multiple
// references to that memory.
// Lifetime is a way of inforcing that a reference is valid.

// lifetimes is a way to make sure memory for a refrence is not cleaned up before
// someone can use it.  otherwise it will cause a compiling error

// insure that memory does not get cleaned up before a refrence can use it.

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

// with lifetimes you spacify that parameter and return have the same lifetime
// lifetimes guarante that the memory in the result will still be valid when return value uses it
// when you handle refrences lifetimes are always used in the background.
// most of the time lifetimes are figured out implicity but there are time when you need to get
// explicit
// you get compiling errors when compiler finds situations where it can't figure out the lifetimes
// implicityly, that's when you need to get explicit with life times.
// for every input reference that does not have an explicityl defined lifetime,
// it will be given it's own lifetime by Rust in the background
// you must guarante that 'b will be alive as long a 'a
// fn get_int_ref<'a, 'b: 'a>(param_1: &'a i32, param_2: &'b i32) -> &'a i32 {
// just guarante that param_1 and param_2 have same lifetime, then this should be valid
fn get_int_ref<'a>(param_1: &'a i32, param_2: &'a i32) -> &'a i32 {
    // all variables called inside this scope will be cleaned
    // let a = 1;
    // &a
    // param_1 // no there is no problem

    // this will cause a problem
    if param_1 > param_2 {
        param_1
    } else {
        param_2
    }
}

// explicityly defining lifetime guaraties that what is returned
// is going to still be around when using it later
fn get_str_ref<'a>(param_1: &'a str, param_2: &'a str) -> &'a str {
    if param_1 > param_2 {
        param_1
    } else {
        param_2
    }
}

// you only need to specifi lifetime if you return a reference.
// it doesn't mater if you have parameters that have lifetimes.
fn test_2<'a>(param_1: &Vec<f64>) -> Vec<f64> {
    param_1.clone()
}

// you use lifetimes when you have refrence inputs and return a reference

// remember that the compiler creates a lifetime for each reference input
// to fix this you need to make sure that lifetimes are the same
fn test_1<'a>(param_1: i32, param_2: &'a str, param_3: &'a str, param_4: f64) -> &'a str {
    if param_1 == 7 && param_4 > 10.0 {
        param_2 // both param_2 and param_3 have same lifetime
    } else {
        param_3 // both param_2 and param_3 have same lifetime
    }
}

// you don't need to spedify lifetimes here.
// rust will do it under the hood
fn test_3(param_1: &str) -> &str {
    param_1
}

// each refrence get's there own lifetimes
// if you do return a refernce the compiler has to know the lifetime of what
// it is returning.
// fn some_function<'a, 'b>(param_1: &'a str, param_2: &'b str) -> &str {
//     todo!()
// }

// const has a lifetime of 'static
// it means that is has the lifetime of the entire program
const SOME_INT: i32 = 30;
// string literanal are always on staitc
// let name = "Sam";
// let name = &'static str = "Sam";
// let name = &str = "Sam";

pub fn lifetimes() {
    let a;
    {
        let b = String::from("Howdy");
        a = b; // a becomes the owner so this is valid
               // a = &b; // this causes a problem because a is just a reference and b disapears after scope
    }

    let some_int_var = 10;
    let some_int_var2 = 20;
    let result_ref = get_int_ref(&some_int_var, &some_int_var2);
    let res2 = get_int_ref(&SOME_INT, &some_int_var2); // const has a lifetime of 'static
    println!("{result_ref}");

    println!("this works: {a}");
    // this will cause a problem
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("r: {}", r);

    // by default this is a staitc lifetime
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
    fn test_lifetimes() {
        lifetimes();
    }
}
