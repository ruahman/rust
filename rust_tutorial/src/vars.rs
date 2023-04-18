use rand::Rng;

// const, get replaces with value, where var is a memory address
#[allow(unused)]
const ID: i32 = 001;

fn get_rand_num() -> i32 {
    // get random number
    let random_num: i32 = rand::thread_rng().gen_range(1..101);
    random_num
}

fn vars() {
    let _let_var = "this should not change";
    // test = "cant't do this"

    let mut _mut_age = 37;
    _mut_age = 40;


    // deconstruct
    let (_decon_name, _decon_age) = ("Brad", 37);

    // shadowing
    let x = 5;
    let x = x + 1;
    {
        #[allow(unused_variables)]
        let x = x * 2;
    }

}

#[allow(unused_variables)]
pub fn demo() {
    println!("--- demo variables ---");
    
    println!("get_rand_num: {}", get_rand_num());
    println!("vars: {:?}", vars());
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
}
