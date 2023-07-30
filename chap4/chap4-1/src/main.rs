// 食事する哲学者問題

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let c0 = Arc::new(Mutex::new(()));    // 箸その1
    let c1 = Arc::new(Mutex::new(()));    // 箸その2

    let c0_p0 = c0.clone();
    let c1_p0 = c1.clone();

    // 哲学者1
    let p0 = thread::spawn(move || {
        for _ in 0..10_000 {
            let _n1 = c0_p0.lock().unwrap();
            let _n2 = c1_p0.lock().unwrap();
            println!("0: eating");
        }
    });

    // 哲学者2
    let p1 = thread::spawn(move || {
        for _ in 0..10_000 {
            let _n1 = c1.lock().unwrap();
            let _n2 = c0.lock().unwrap();
            println!("1: eating");
        }
    });

    p0.join().unwrap();
    p1.join().unwrap();
}