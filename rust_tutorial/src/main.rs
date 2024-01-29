#![allow(dead_code)]
#![allow(unused_imports)]

mod conditions;
mod enumerations;
mod hello_world;
mod structs;
mod variables;

fn main() {
    hello_world::run();
    variables::run();
    conditions::run();
    variables::run();
    enumerations::run();
}
