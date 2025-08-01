#![allow(dead_code)]

use cucumber::{given, then, when, World};

// These `Cat` definitions would normally be inside your project's code,
// not test code, but we create them here for the show case.
#[derive(Debug, Default)]
struct Cat {
    pub hungry: bool,
}

impl Cat {
    fn feed(&mut self) {
        self.hungry = false;
    }
}

// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
// but sometime Default::default() is not enough.
// in such case a custome consructor may be specified via
// [world(init = my_constructor)]
// you can specify a constructor if you like
// a world object is created for scenario and gets changes on each step
#[derive(Debug, Default, World)]
pub struct AnimalWorld {
    cat: Cat,
}

// give is used to initialize the world of the scenario
// #[given("a hungry cat")]
// fn hungry_cat(world: &mut AnimalWorld) {
//     // world is initialized here
//     world.cat.hungry = true;
// }

// #[given(regex = r"^a (hungry|satiated) cat$")]
// fn hungry_cat(world: &mut AnimalWorld, state: String) {
//     match state.as_str() {
//         "hungry" => world.cat.hungry = true,
//         "satiated" => world.cat.hungry = false,
//         _ => unreachable!(),
//     }
// }

// you can also use cucumber expressions
#[given(expr = "a {word} cat")]
fn hungry_cat(world: &mut AnimalWorld, state: String) {
    match state.as_str() {
        "hungry" => world.cat.hungry = true,
        "satiated" => world.cat.hungry = false,
        _s => panic!("expected 'hungry' or 'satiated', found: {}", _s),
    }
}

// when is used for firing event or action we are testing
#[when("I feed the cat")]
fn feed_cat(world: &mut AnimalWorld) {
    // result is saved in the world
    world.cat.feed();
}

// this is verifies the result
#[then("the cat is not hungry")]
fn cat_is_fed(world: &mut AnimalWorld) {
    // validate what is in world
    assert!(!world.cat.hungry);
}

// This runs before everything else, so you can setup things here.
#[tokio::main]
async fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    // futures::executor::block_on(AnimalWorld::run("tests/features/book/animal.feature"));
    AnimalWorld::run("tests/features/book/animal.feature").await;
}
