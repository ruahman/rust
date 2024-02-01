mod arrays;
mod conditions;
mod enumerations;
mod hello_world;
mod options;
mod structs;
mod tuples;
mod unions;
mod variables;

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
}
