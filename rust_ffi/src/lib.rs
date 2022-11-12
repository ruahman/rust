// use c ABI(Application Binary Interface)

#[no_mangle] // make sure name 'add' doesn't get thrown away
pub extern "C" fn add(x: f64, y: f64) -> f64 {
    x + y
}

