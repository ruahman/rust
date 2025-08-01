
```
Feature: Eating too much cucumbers may not be good for you

  Scenario: Eating a few isn't a problem
    Given Alice is hungry
    When she eats 3 cucumbers
    Then she is full
```

```rust 
#[given(expr = "{word} is hungry")] // Cucumber Expression
async fn someone_is_hungry(w: &mut World, user: String) {
    sleep(Duration::from_secs(2)).await;

    w.user = Some(user);
}

#[when(regex = r"^(?:he|she|they) eats? (\d+) cucumbers?$")]
async fn eat_cucumbers(w: &mut World, count: usize) {
    sleep(Duration::from_secs(2)).await;

    w.capacity += count;

    assert!(w.capacity < 4, "{} exploded!", w.user.as_ref().unwrap());
}

#[then("she is full")]
async fn is_full(w: &mut World) {
    sleep(Duration::from_secs(2)).await;

    assert_eq!(w.capacity, 3, "{} isn't full!", w.user.as_ref().unwrap());
}

```

