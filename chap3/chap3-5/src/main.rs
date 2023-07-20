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
        println!("func1: {}", val);
    }
}

fn func2(lock: Arc<Mutex<u64>>) {
    for _ in 11..20 {
        let mut val = lock.lock().unwrap();
        *val += 2;
        println!("func2 :{}", val);
    }
}

fn main() {
    let lock0 = Arc::new(Mutex::new(0));    // 参照カウンター型のスマートポインタ
    let lock1 = lock0.clone();

    let th0 = thread::spawn(move || {
        func1(lock0)
    });

    let th1 = thread::spawn(move || {
        func2(lock1)
    });

    th0.join().unwrap();
    th1.join().unwrap();
}
