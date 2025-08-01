keywords can be used to run the same scenario multiple times, 
with differnt combinations of values

```
Feature: Animal feature

  Scenario Outline: If we feed a hungry animal it will no longer be hungry
    Given a hungry <animal>
    When I feed the <animal> <n> times
    Then the <animal> is not hungry

  Examples: 
    | animal | n |
    | cat    | 2 |
    | dog    | 3 |
    | 🦀     | 4 |

```

```rust
#[derive(Debug, Default)]
struct Animal {
    pub hungry: bool,
}

impl Animal {
    fn feed(&mut self) {
        self.hungry = false;
    }
}

#[derive(Debug, Default, World)]
pub struct AnimalWorld {
    animals: HashMap<String, Animal>,
}

#[given(regex = r"^a (hungry|satiated) (\S+)$")]
async fn hungry_animal(world: &mut AnimalWorld, state: String, which: String) {
    sleep(Duration::from_secs(2)).await;

    world.animals.entry(which).or_insert(Animal::default()).hungry =
        match state.as_str() {
            "hungry" => true,
            "satiated" => false,
            _ => unreachable!(),
        };
}

#[when(expr = "I feed the {word} {int} time(s)")]
async fn feed_animal(world: &mut AnimalWorld, which: String, times: usize) {
    sleep(Duration::from_secs(2)).await;

    for _ in 0..times {
        world.animals.get_mut(&which).map(Animal::feed);
    }
}

#[then(expr = "the {word} is not hungry")]
async fn animal_is_fed(world: &mut AnimalWorld, which: String) {
    sleep(Duration::from_secs(2)).await;

    assert!(!world.animals.get(&which).map_or(true, |a| a.hungry));
}

```
