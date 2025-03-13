// atomic reference counter
use std::sync::Arc;
// mutualy exclusive assessor
use std::sync::Mutex;
use std::thread;

struct Bank {
    balance: f32,
}

fn withdraw(bank: &Arc<Mutex<Bank>>, amount: f32) {
    // before we can access the bank, we need to lock it
    let mut bank = bank.lock().unwrap();

    if bank.balance < 5.0 {
        println!("balance is low: {}", bank.balance);
        println!("can't withdraw anymore money")
    } else {
        bank.balance -= amount;
        println!("withdrew: {}", amount);
        println!("new balance: {}", bank.balance);
    }
}

pub fn run() {
    let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank { balance: 100.0 }));

    // make a bunch of threads that withdraw money
    let handles = (0..10).map(|_| {
        let bank = Arc::clone(&bank);
        thread::spawn(move || {
            withdraw(&bank, 5.0);
        })
    });

    // wait for all the threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // show the final balance
    println!("balance: {}", bank.lock().unwrap().balance)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bank() {
        run();
    }
}
