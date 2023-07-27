// mod arrays;
// mod cli;
// mod closures;
// mod collections;
// mod conditions;
// mod enums;
// mod functions;
// mod generics;
// mod hashmaps;
// mod loops;
// mod modules;
// mod options;
// mod ownership;
// mod pointers_ref;
// mod print;
// mod reference_counter;
// mod slices;
// mod smart_pointers;
// mod strings;
// mod structs;
// mod threads;
// mod traits;
// mod tuple;
// mod types;
mod data_types;
mod variables;
// mod vectors;
mod guess_random_number;
// mod io;
use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long)]
    guess: bool,

    #[arg(short, long)]
    variables: bool,

    #[arg(short, long)]
    data_types: bool,
}

// cargo run -- --help
fn main() {
    let args = Cli::parse();
    if args.guess {
        guess_random_number::run();
    } else if args.variables {
        variables::exec();
    } else if args.data_types {
        data_types::exec();
    } else {
        println!("Please specify a subcommand");
    }

    // guess_random_number::demo();
    // variables::exec();
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
}
