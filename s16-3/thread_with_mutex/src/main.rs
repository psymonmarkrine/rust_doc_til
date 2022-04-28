use std::sync::Mutex;
use std::thread;
use std::rc::Rc;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
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
