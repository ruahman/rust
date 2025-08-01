
occasionally, we may find ourselves repeating the same Given step in all the scenarios 

we can move such a Given step to the background.
which are run before each scenario 

```
Feature: Animal feature
    
  Background: 
    Given a hungry cat
    
  Rule: Hungry cat becomes satiated
      
    Scenario: If we feed a hungry cat it will no longer be hungry
      When I feed the cat
      Then the cat is not hungry
    
  Rule: Satiated cat remains the same
      
    Background:
      When I feed the cat

    Scenario: If we feed a satiated cat it will not become hungry
      When I feed the cat
      Then the cat is not hungry

```

```rust 
#[given(regex = r"^a (hungry|satiated) cat$")]
async fn hungry_cat(world: &mut AnimalWorld, state: String) {
    sleep(Duration::from_secs(2)).await;

    match state.as_str() {
        "hungry" => world.cat.hungry = true,
        "satiated" => world.cat.hungry = false,
        _ => unreachable!(),
    }
}

#[when("I feed the cat")]
async fn feed_cat(world: &mut AnimalWorld) {
    sleep(Duration::from_secs(2)).await;

    world.cat.feed();
}

#[then("the cat is not hungry")]
async fn cat_is_fed(world: &mut AnimalWorld) {
    sleep(Duration::from_secs(2)).await;

    assert!(!world.cat.hungry);
}

```
