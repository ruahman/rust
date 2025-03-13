// atomic reference count
// mutualy exclusive assessor
use std::sync::{Arc, Mutex};
use std::thread;

struct Person {
    // if you just want multiple thread to read from a var use Arc
    name: Arc<String>,

    // if you want multiple thread to read and write to a variable then use Arc
    // and Mutex
    state: Arc<Mutex<String>>,
}

impl Person {
    fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> Self {
        Self { name, state }
    }
}
pub fn run() {
    let name = Arc::new("John".to_string());
    let state = Arc::new(Mutex::new("sad".to_string()));
    let p = Person::new(name.clone(), state.clone());

    // you can do this because you are using a reference counter
    // multiple threads can access the same memory
    // both name and p.name point to the same memory because they are using a reference counter 
    // atomic refrence counter is a refrence counter that works between threads
    // it make the memory access thread safe
    println!("{}", name);
    println!("{}", p.name);

    // but to change the memory you need to use a  mutex
    // so that you can guarante that only one thread at a time and change the memory value
    println!("{}", p.state.lock().unwrap().as_str());

    // more than one thread have access to rc counter and it's safe
    thread::spawn(move || {
        println!("{}", p.name);

        // only one thread at a time can modify the state,
        // Arc only allow multiple threads to access memeory
        // but to change memory a mutex has to be used to make sure only one thread
        // at a time has access to it.
        
        // lock the mutex before you can change anything
        let mut state = p.state.lock().unwrap();
        state.clear();
        state.push_str("happy");
    })
    .join()
    .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_atomic_reference_counter() {
        run();
    }
}
