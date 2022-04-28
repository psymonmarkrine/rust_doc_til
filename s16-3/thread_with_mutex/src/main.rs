use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let hundle = thread::spawn(move|| {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(hundle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("result: {}", *counter.lock().unwrap());
}
