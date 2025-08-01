
rule represent a business rule that should be implemented.  it provides additional information 
for a feature.

a rule is used to group together several cenarios belonging to the buisness rule. 

a rule should take one or more scenarios

no additional work is required on the implementation side. 

```
Feature: Animal feature
    
  Rule: Hungry cat becomes satiated
      
    Scenario: If we feed a hungry cat it will no longer be hungry
      Given a hungry cat
      When I feed the cat
      Then the cat is not hungry
    
  Rule: Satiated cat remains the same
      
    Scenario: If we feed a satiated cat it will not become hungry
      Given a satiated cat
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
