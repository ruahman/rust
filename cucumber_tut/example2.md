
```gerkin 
Feature: Eating too much cucumbers may not be good for you
    
  Scenario: Eating a few isn't a problem
    Given Alice is hungry
    When she eats 3 cucumbers
    Then she is full

```

```rust 
use std::time::Duration;

use cucumber::{given, then, when, World as _};
use tokio::time::sleep;

// each senario create an instance of this
#[derive(cucumber::World, Debug, Default)]
struct World {
    user: Option<String>,
    capacity: usize,
}

// setup and initialize world
#[given(expr = "{word} is hungry")] // Cucumber Expression
async fn someone_is_hungry(w: &mut World, user: String) {
    sleep(Duration::from_secs(2)).await;
    
    w.user = Some(user);
}

// trigger event and save result in world
#[when(regex = r"^(?:he|she|they) eats? (\d+) cucumbers?$")]
async fn eat_cucumbers(w: &mut World, count: usize) {
    sleep(Duration::from_secs(2)).await;

    w.capacity += count;
    
    assert!(w.capacity < 4, "{} exploded!", w.user.as_ref().unwrap());
}

// verify result by seeing what was saved in world 
#[then("she is full")]
async fn is_full(w: &mut World) {
    sleep(Duration::from_secs(2)).await;

    assert_eq!(w.capacity, 3, "{} isn't full!", w.user.as_ref().unwrap());
}

#[tokio::main]
async fn main() {
    World::run("tests/features/readme").await;
}
```

add test to Cargo.toml
```toml
[[test]]
name = "readme"
harness = false  # allows Cucumber to print output instead of libtest
```
