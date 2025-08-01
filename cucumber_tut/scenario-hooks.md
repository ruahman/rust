scenario hooks represent code running for each scenario and not visible in .feature 

add before hook 
```rust
World::cucumber()
    .before(|_feature, _rule, _scenario, _world| {
        time::sleep(Duration::from_millis(300)).boxed_local()
    })
    .run_and_exit("tests/features/book");

```

add after hook
```rust 
World::cucumber()
    .after(|_feature, _rule, _scenario, _ev, _world| {
        time::sleep(Duration::from_millis(300)).boxed_local()
    })
    .run_and_exit("tests/features/book");

```
