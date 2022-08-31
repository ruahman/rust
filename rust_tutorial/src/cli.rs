use std::env::{args, Args};

pub fn demo(){
    let argsx: Vec<String> = args().collect();

    println!("Args: {:?}", argsx);

    let mut argsy: Args = args();

    let first = argsy.nth(0);
    println!("Args nth: {:?}",first);

    // let mut name: String = String::new();
    // std::io::stdin().read_line(buf: &mut name);
    // println!("{}",name);
    
}
