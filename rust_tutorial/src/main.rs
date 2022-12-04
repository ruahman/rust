// mod arrays;
// mod cli;
// mod closures;
// mod collections;
// mod conditions;
// mod enums;
// mod functions;
// mod generics;
// mod hashmaps;
// mod guess_random_number;
// mod io;
// mod loops;
// mod modules;
// mod options;
// mod ownership;
// mod pointers_ref;
// mod print;
// mod reference_counter;
// mod smart_pointers;
// mod strings;
// mod slices;
// mod structs;
// mod threads;
// mod traits;
// mod tuple;
// mod types;
// mod vectors;
mod vars;

fn main() {
    println!("----- rust tutorial -----");

    vars::demo();
    // io::demo();
    // tuple::demo();
    // arrays::demo();
    // conditions::demo();
    // ownership::demo();
    // slices::demo();
    // print::demo();
    // types::demo();
    // strings::demo();
    // vectors::demo();
    // loops::demo();
    // functions::demo();
    // pointers_ref::demo();
    // structs::demo();
    // enums::demo();
    // cli::demo();
    // generics::demo();
    // hashmaps::demo();
    // traits::demo();
    // modules::demo();
    // closures::demo();
    // smart_pointers::demo();
    // threads::demo();
    // options::demo();
    // collections::demo();
    // reference_counter::demo();
    // guess_random_number::demo();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4)
    }
}
