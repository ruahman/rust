use std::ffi::CStr;
use std::i8;

#[no_mangle]
pub extern "C" fn add(x: f64, y: f64) -> f64 {
    x + y
}

#[no_mangle]
pub extern "C" fn print_string(str_ptr: *const i8) -> () {
    let my_string: &str;
    unsafe {
        my_string = CStr::from_ptr(str_ptr)
            .to_str()
            .expect("string could not be parsed");
    }
    println!("The string is {}", my_string);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2.0, 2.0);
        assert_eq!(result, 4.0);
    }
}
