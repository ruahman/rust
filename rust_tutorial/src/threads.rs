use std::thread;
use std::time::Duration;

pub fn run() {
    let thread1 = thread::spawn(|| {
        for i in 1..25 {
            println!("thread: {}", i);
            thread::sleep(Duration::from_millis(100))
        }
    });

    for i in 1..20 {
        println!("main: {}", i);
        thread::sleep(Duration::from_millis(100))
    }

    thread1.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_threads() {
        run();
    }
}
