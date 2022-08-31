use rand::Rng;

mod arrays;
mod cli;
mod closures;
mod conditions;
mod enums;
mod functions;
mod generics;
mod hashmaps;
mod loops;
mod modules;
mod ownership;
mod pointers_ref;
mod print;
mod smart_pointers;
mod strings;
mod structs;
mod threads;
mod traits;
mod tuple;
mod types;
mod vars;
mod vectors;

fn main() {
    println!("hello word");

    let random_num: i32 = rand::thread_rng().gen_range(1..101);
    println!("{}", random_num);

    print::demo();
    vars::demo();
    types::demo();
    strings::demo();
    tuple::demo();
    vectors::demo();
    conditions::demo();
    loops::demo();
    functions::demo();
    pointers_ref::demo();
    arrays::demo();
    structs::demo();
    enums::demo();
    cli::demo();
    generics::demo();
    ownership::demo();
    hashmaps::demo();
    traits::demo();
    modules::demo();
    closures::demo();
    smart_pointers::demo();
    threads::demo();
}
