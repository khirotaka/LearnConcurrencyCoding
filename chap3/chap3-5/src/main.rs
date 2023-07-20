use std::sync::{Arc, Mutex};
use std::thread;

fn func1(lock: Arc<Mutex<u64>>) {
    /*loop {
        let mut val = lock.lock().unwrap();
        *val += 1;
        println!("{}", val);
    }*/
    for _ in 0..10 {
        let mut val = lock.lock().unwrap();
        *val += 1;
        println!("{}", val);
    }
}

fn main() {
    let lock0 = Arc::new(Mutex::new(0));    // 参照カウンター型のスマートポインタ
    let lock1 = lock0.clone();

    let th0 = thread::spawn(move || {
        func1(lock0)
    });

    let th1 = thread::spawn(move || {
        some_func(lock1)
    });

    th0.join().unwrap();
    th1.join().unwrap();
}
