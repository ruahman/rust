use cucumber::{given, then, when, World};

// Define the world (state) for your tests
#[derive(Debug, World)]
#[world(init = MyWorld::new)]
struct MyWorld {
    number: i32,
    result: i32,
}

impl MyWorld {
    fn new() -> Self {
        Self {
            number: 0,
            result: 0,
        }
    }
}

#[given(regex = r"a number (\d+)")]
fn set_number(world: &mut MyWorld, number: i32) {
    world.number = number;
}

#[when(regex = r"I add (\d+) to it")]
fn add_number(world: &mut MyWorld, number: i32) {
    world.result = world.number + number;
}

#[then(regex = r"the result should be (\d+)")]
fn check_result(world: &mut MyWorld, expected: i32) {
    assert_eq!(world.result, expected);
}

#[tokio::main]
async fn main() {
    futures::executor::block_on(MyWorld::run("tests/features/example2.feature"));
}
