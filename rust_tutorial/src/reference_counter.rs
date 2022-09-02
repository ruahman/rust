// use std::rc::Rc;
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
pub fn demo() {
    let name = Arc::new("John".to_string());
    let state = Arc::new(Mutex::new("sad".to_string()));
    let p = Person::new(name.clone(), state.clone());
    println!("{}", p.name);
    // noy you can do this because you are using a reference counter
    println!("{}", name);
    println!("{}", p.name);

    println!("{}", p.state.lock().unwrap().as_str());

    // more than one thread have access to rc counter and it's safe
    thread::spawn(move || {
        println!("{}", p.name);

        // only one thread at a time can modify the state,
        // Arc only allow multiple threads to access memeory but to change
        // memory a mutex has to be used to make sure only one thread
        // at a time has access to it.
        let mut state = p.state.lock().unwrap();
        state.clear();
        state.push_str("happy");
        // p.state.clear();
        // p.state.push_str(", happy");
    })
    .join()
    .unwrap();
}
