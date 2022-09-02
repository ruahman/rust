// 1. each value has an owner
// 2. there is only one owner at a time
// 3. when the owner goes out of scope the value disapears too

fn print_str(x: String) {
    println!("{}", x);
}
fn print_return_str(x: String) -> String {
    println!("{}", x);
    x
}
fn change_string(name: &mut String) {
    name.push_str("is happy");
    println!("{}", name);
}

// this just bowwors the vec it doesn't become the new owner
fn print_vector(v: &Vec<i32>) {
    println!("{:?}", v);
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

pub fn demo() {
    let str1 = String::from("hello world");
    let str2 = str1;
    let str3 = str2.clone();
    // this causes problems
    // println!("{}", str1);
    println!("{}{}", str2, str3);
    // print_str(str1);

    // problem because it got moved to function
    // println!("{}",str1)
    let mut str4: String = String::from("test ");
    change_string(&mut str4);
    print_str(str4);

    let str5 = print_return_str(str3);
    print_return_str(str5);
    // let str4 = print_return_str(str1)

    let a = String::from("test");
    {
        let b = a;
        println!("{}", b)
    }
    // can do this because ownership was moved to b
    // println!("{}", a);

    // stack, heep

    let x = Box::new(45);
    {
        let y = x;
        println!("{}", y);
    }
    // cant do this because everthing is on the heep
    // println!("{}", x);

    let g = 42;
    {
        let h = g;
        println!("{}", h);
    }
    // this is fine because values are on the stack
    println!("{}", g);

    // v is the owner of vector
    let v = vec![1, 2, 3];
    // v2 is now the owner
    let v2 = v;
    println!("{:?}", v2);

    print_vector(&v2);

    println!("{:?}", v2);

    let mut m = vec![1, 2, 3];

    mod_vector(&mut m);

    print_vector(&m);
    // borrowing
    let boss = Person {
        name: String::from("Elon Must"),
    };
    let tesla = Company {
        name: String::from("Tesla"),
        ceo: &boss,
    };

    println!("{}{}", tesla.name, tesla.ceo.name);
}
