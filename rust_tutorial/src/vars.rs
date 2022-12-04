use rand::Rng;

fn get_rand_num() -> i32 {
    // get random number
    let random_num: i32 = rand::thread_rng().gen_range(1..101);
    random_num
}

#[allow(dead_code)]
#[derive(Debug)]
struct MutVars {
    let_var: &'static str,
}
fn mut_vars() -> MutVars {
    let let_var = "this should not change";
    // test = "cant't do this"
    return MutVars { let_var };
}

#[allow(unused_variables)]
pub fn demo() {
    println!("--- demo variables ---");

    println!("{}", get_rand_num());
    println!("{:?}", mut_vars());
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

    // deconstruct
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

#[cfg(test)]
mod vars_tests {
    use super::*;

    #[test]
    fn test_random() {
        let res1 = get_rand_num();
        let res2 = get_rand_num();
        assert_ne!(res1, res2);
    }

    #[test]
    fn test_mutable_vars() {
        let res = mut_vars();
        assert_eq!(res.let_var, "this should not change")
    }
}
