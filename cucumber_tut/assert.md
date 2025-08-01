
there are two ways of doing assertions
* throwing a panic
* returning an error

# panic 
```rust 
#[then("the cat is not hungry")]
fn cat_is_fed(_: &mut AnimalWorld) {
    panic!("Cats are always hungry!")
}
```

# result 
```rust 
#[when("I feed the cat")]
fn feed_cat(_: &mut AnimalWorld) {}

#[then("the cat is not hungry")]
fn cat_is_fed(world: &mut AnimalWorld) -> Result<(), &'static str> {
    (!world.cat.hungry).then_some(()).ok_or("Cat is still hungry!")
}

```
