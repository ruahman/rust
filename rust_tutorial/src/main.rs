mod arrays;
// mod cli;
// mod closures;
// mod collections;
mod conditions;
mod enums;
mod functions;
mod generics;
mod hashmaps;
mod loops;
mod modules;
// mod options;
mod ownership;
// mod pointers_ref;
// mod print;
// mod reference_counter;
// mod slices;
// mod smart_pointers;
mod strings;
mod structs;
// mod threads;
mod traits;
mod tuple;
// mod types;
mod data_types;
mod variables;
mod vectors;
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

    #[arg(short, long)]
    conditions: bool,

    #[arg(short, long)]
    arrays: bool,

    #[arg(short, long)]
    loops: bool,

    #[arg(short, long)]
    tuple: bool,

    #[arg(short, long)]
    string: bool,

    #[arg(short, long)]
    enums: bool,

    #[arg(long)]
    vectors: bool,

    #[arg(short, long)]
    functions: bool,

    #[arg(long)]
    generics: bool,

    #[arg(long)]
    hashmaps: bool,

    #[arg(long)]
    structs: bool,

    #[arg(long)]
    traits: bool,

    #[arg(short, long)]
    modules: bool,
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
    } else if args.conditions {
        conditions::exec();
    } else if args.arrays {
        arrays::exec();
    } else if args.loops {
        loops::exec()
    } else if args.tuple {
        tuple::exec()
    } else if args.string {
        strings::exec()
    } else if args.enums {
        enums::exec()
    } else if args.vectors {
        vectors::exec()
    } else if args.functions {
        functions::exec()
    } else if args.generics {
        generics::exec()  
    } else if args.hashmaps {
        hashmaps::exec()
    } else if args.structs {
        structs::exec()
    } else if args.traits {
        traits::exec()
    } else if args.modules {
        modules::exec()
    }else {
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
