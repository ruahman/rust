use rand::Rng;

#[allow(unused_variables)]
pub fn demo() {
    println!("***** demo variables *****");

    // get random number
    let random_num: i32 = rand::thread_rng().gen_range(1..101);
    println!("{}", random_num);

    // regualr var
    let my_name = "diego";
    println!("my name is {}", my_name);

    // mutable var
    let mut age = 37;
    println!("{}", age);
    age = 40;
    println!("{}", age);

    // const, get replaces with value, where var is a memory address
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // deconstuct
    let (my_namex, my_age) = ("Brad", 37);
    println!("{}{}", my_namex, my_age);

    // shadowing
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();
}
