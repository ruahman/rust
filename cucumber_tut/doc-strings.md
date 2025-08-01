
doc string provide a way to pass a large piece of text to a step.

```
Feature: Animal feature
    
  Scenario: If we feed a hungry Simba it will no longer be hungry
    Given a hungry cat
      """markdown
      About Simba
      ===========
      A hungry cat called Simba is rescued from a Whiskas tin in a calamitous
      mash-up of cat food brands.
      """
    When I feed the cat
    Then the cat is not hungry

```

```rust 
#[given(regex = r"^a (hungry|satiated) cat$")]
async fn hungry_cat(world: &mut AnimalWorld, step: &Step, state: String) {
    // Feed only Leo and Felix.
    if !step
        .docstring // get doc string from here
        .as_ref()
        .map_or(false, |text| text.contains("Felix") || text.contains("Leo"))
    {
        panic!("Only Felix and Leo can be fed");
    }

    match state.as_str() {
        "hungry" => world.cat.hungry = true,
        "satiated" => world.cat.hungry = false,
        _ => unreachable!(),
    }
}

```
