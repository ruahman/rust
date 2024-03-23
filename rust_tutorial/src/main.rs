mod arrays;
mod closures;
mod conditions;
mod enumerations;
mod functions;
mod generics;
mod hashmaps;
mod hashsets;
mod hello_world;
mod iterators;
mod methods;
mod options;
mod pattern_matching;
mod strings;
mod structs;
mod tuples;
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
}
