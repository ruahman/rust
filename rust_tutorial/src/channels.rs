use std::sync::mpsc;
use std::thread;

pub fn run() {
    println!("channels.rs");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    // this will block until a value is sent
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    let (tx2, rx2) = mpsc::channel();
    let tx3 = tx2.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hi3"),
            String::from("from3"),
            String::from("the3"),
            String::from("thread3"),
        ];

        for val in vals {
            tx3.send(val).unwrap();
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    for received in rx2 {
        println!("Got: {}", received);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_channels() {
        run();
    }
}
