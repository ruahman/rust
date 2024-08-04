mod arrays;
mod atomic_reference_counter;
mod bank;
mod channels;
mod closures;
mod collections;
mod conditions;
mod data_types;
mod enumerations;
mod enums;
mod error_handling;
mod formatting;
mod functions;
mod generics;
mod guess_random_number;
mod hashmaps;
mod hashsets;
mod hello_world;
mod io;
mod iterators;
mod lifetime;
mod loops;
mod methods;
mod modules;
mod mutex;
mod options;
mod ownership;
mod pattern_matching;
mod pointers_ref;
mod print;
mod refcell;
mod reference_counter;
mod results;
mod slices;
mod smart_pointers;
mod strings;
mod structs;
mod threads;
mod trait_objects;
mod traits;
mod tuples;
mod types;
mod unions;
mod variables;
mod vectors;

fn main() {
    hello_world::run();
    variables::run();
    conditions::run();
    variables::run();
    enumerations::run();
    unions::run();
    options::run();
    arrays::run();
    tuples::run();
    pattern_matching::run();
    generics::run();
    strings::run();
    hashmaps::run();
    hashsets::run();
    iterators::run();
    functions::run();
    methods::run();
    closures::run();
    traits::run();
    formatting::run();
    ownership::run();
    lifetime::run();
    reference_counter::run();
    atomic_reference_counter::run();
    threads::run();
    modules::run();
    error_handling::run();
    smart_pointers::run();
    bank::run();
    enums::run();
    slices::run();
    results::run();
    collections::run();
    data_types::run();
    types::run();
    print::run();
    loops::run();
    pointers_ref::run();
    refcell::run();
    channels::run();
    mutex::run();
    trait_objects::run();
    io::run();
    guess_random_number::run();
}
