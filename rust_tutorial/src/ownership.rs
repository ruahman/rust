#[allow(dead_code)]
#[derive(Debug)]
struct Ownership {
    str2: String,
    str3: String,
    str6: String,
    g: i32,
    v2: Vec<i32>,
    m: Vec<i32>,
}
fn ownership() -> Ownership {
    // 1. each value has an owner
    // 2. there is only one owner at a time
    // 3. when the owner goes out of scope the value disapears too

    // only one can point to a reference
    let str1 = String::from("hello world");
    let str2 = str1;
    let str3 = str2.clone();
    // this causes problems because str1 no longer point to "hello world"
    // println!("{}", str1);

    let str4 = str3.clone();
    print_str(str4);
    // problem because it got moved to function
    // println!("{}", str4);

    let mut str5: String = String::from("test ");
    change_string(&mut str5);

    let str6 = print_return_str(str5);

    let a = String::from("test");
    {
        let b = a;
        println!("{}", b);
        // memory was released as soon as it left scope
    }
    // cant do this because ownership was moved to b
    // println!("{}", a);

    let x = Box::new(45);
    {
        let y = x;
        println!("{}", y);
        // memory was released as soon as it left scope
    }
    // cant do this because everthing is on the heep
    // println!("{}", x);

    let g = 42;
    {
        let h = g;
        println!("{}", h);
    }
    // this is fine because values are on the stack, there is no borrow checking
    // for values on the stack
    println!("{}", g);

    // v is the owner of vector
    let v = vec![1, 2, 3];
    // v2 is now the owner
    let v2 = v;
    println!("{:?}", v2);

    let mut m = vec![1, 2, 3];

    mod_vector(&mut m);

    // they must have the same lifescycle
    let boss = Person {
        name: String::from("Elon Must"),
    };
    let tesla = Company {
        name: String::from("Tesla"),
        ceo: &boss,
    };

    println!("{}{}", tesla.name, tesla.ceo.name);

    // pass by reference
    let s1 = String::from("hello");
    let len = calculate_lenght(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // mutable reference
    let mut s = String::from("hello");

    change(&mut s);

    return Ownership {
        str2,
        str3,
        str6,
        g,
        v2,
        m,
    };
}

fn print_str(x: String) {
    println!("{}", x);
}
fn print_return_str(x: String) -> String {
    println!("{}", x);
    x
}
fn change_string(name: &mut String) {
    name.push_str("is happy");
}

fn mod_vector(v: &mut Vec<i32>) {
    v.push(23);
}

struct Person {
    name: String,
}

// make sure ceo has same lifetime as Company,
// ceo must have same lifetime as Company otherwise there will be
// a compile error
struct Company<'z> {
    name: String,
    ceo: &'z Person,
}

fn calculate_lenght(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

pub fn demo() {
    println!("ownership: {:?}", ownership())
}

#[cfg(test)]
mod ownership_tests {
    use super::*;
    #[test]
    fn test_ownership() {
        let result = ownership();
        assert_eq!(result.str2, "hello world");
        assert_eq!(result.str6, "test is happy");
        assert_eq!(result.g, 42);
        assert_eq!(result.v2, vec![1, 2, 3]);
        assert_eq!(result.m, vec![1, 2, 3, 23]);
    }
}
