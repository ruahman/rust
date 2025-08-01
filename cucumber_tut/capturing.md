
capture a string using regex 
```rust 
#[given(regex = r"^a (hungry|satiated) cat$")]
fn hungry_cat(world: &mut AnimalWorld, state: String) {
    match state.as_str() {
        "hungry" =>  world.cat.hungry = true,
        "satiated" =>  world.cat.hungry = false,
        _ => unreachable!(),
    }
}

#[when(regex = r"^I feed the cat \d+ times?$")]
fn feed_cat(world: &mut AnimalWorld) {
    world.cat.feed();
}
```

# FromStr 

capture the string but then use the FromStr function to create an object
```rust 
#[derive(Debug, Default)]
enum State {
    Hungry,
    #[default]
    Satiated,
}

impl FromStr for State {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "hungry" => Self::Hungry,
            "satiated" => Self::Satiated,
            invalid => return Err(format!("Invalid `State`: {invalid}")),
        })
    }
}

#[given(regex = r"^a (hungry|satiated) cat$")]
fn hungry_cat(world: &mut AnimalWorld, state: State) {
    world.cat.hungry = state;
}

#[when(regex = r"^I feed the cat (\d+) times?$")]
fn feed_cat(world: &mut AnimalWorld, times: u8) {
    for _ in 0..times {
        world.cat.feed();
    }
}
```

# Cucumber Expressions 

```rust 
#[given(expr = "a {word} cat")]
fn hungry_cat(world: &mut AnimalWorld, state: State) {
    world.cat.hungry = state;
}

#[when(expr = "I feed the cat {int} time(s)")]
fn feed_cat(world: &mut AnimalWorld, times: u8) {
    for _ in 0..times {
        world.cat.feed();
    }
}

```


# Custom parameters 

you can create a parameter that just maps to the string

```rust 
use cucumber::Parameter;

#[derive(Debug, Default, Parameter)]
// NOTE: `name` is optional, by default the lowercased type name is implied.
#[param(name = "hungriness", regex = "hungry|satiated")]
enum State {
    Hungry,
    #[default]
    Satiated,
}

// NOTE: `Parameter` requires `FromStr` being implemented.
impl FromStr for State {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "hungry" => Self::Hungry,
            "satiated" => Self::Satiated,
            invalid => return Err(format!("Invalid `State`: {invalid}")),
        })
    }
}

#[given(expr = "a {hungriness} cat")]
fn hungry_cat(world: &mut AnimalWorld, state: State) {
    world.cat.hungry = state;
}

```
