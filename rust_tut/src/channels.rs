#![allow(dead_code)]

use std::sync::mpsc;
use std::thread;

pub fn run() {
    println!("channels.rs");

    // transmitter and receiver
    let (tx, rx) = mpsc::channel();

    // transmit "hi"
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    // receive "hi"
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // you can have multiple transmiters
    let (tx2, rx2) = mpsc::channel();
    let tx3 = tx2.clone();

    // transmitter 1
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

    // transmitter 2
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

    // receive messages from both transmitters
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
