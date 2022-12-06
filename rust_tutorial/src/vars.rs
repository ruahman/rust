use rand::Rng;

// const, get replaces with value, where var is a memory address
const ID: i32 = 001;

fn get_rand_num() -> i32 {
    // get random number
    let random_num: i32 = rand::thread_rng().gen_range(1..101);
    random_num
}

#[allow(dead_code)]
#[derive(Debug)]
struct MutVars {
    let_var: &'static str,
    mut_age: i32,
    constant_id: i32,
    decon_name: &'static str,
    decon_age: i32,
    shaddow: i32,
}
fn vars() -> MutVars {
    let let_var = "this should not change";
    // test = "cant't do this"

    #[allow(unused_assignments)]
    let mut mut_age = 37;
    mut_age = 40;

    // deconstruct
    let (decon_name, decon_age) = ("Brad", 37);

    // shadowing
    let x = 5;

    let x = x + 1;

    {
        #[allow(unused_variables)]
        let x = x * 2;
    }

    return MutVars {
        let_var,
        mut_age,
        constant_id: ID,
        decon_name,
        decon_age,
        shaddow: x,
    };
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

    #[test]
    fn test_mutable_vars() {
        let res = vars();
        assert_eq!(res.let_var, "this should not change");
        assert_eq!(res.mut_age, 40);
        assert_eq!(res.constant_id, ID);
        assert_eq!(res.shaddow, 6);
    }
}
