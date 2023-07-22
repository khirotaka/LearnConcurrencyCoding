use std::sync::{Arc, RwLock};
use std::thread;

fn reader(lock: Arc<RwLock<i32>>, id: usize) {
    loop {
        // 読み取りロックなので、他の読み取りロックと共有できる
        let v = lock.read().unwrap();
        println!("reader-{}: {}", id, v);
    }
}

fn writer(lock: Arc<RwLock<i32>>) {
    loop {
        // 書き込みロックは排他的なため、誰も読み取り・書き込みロックを獲得していない時に獲得できる
        let mut v = lock.write().unwrap();
        *v += 1;
        println!("write: {}", v);
    }
}

fn main() {
    let lock0 = Arc::new(RwLock::new(0));
    let lock1 = lock0.clone();
    let lock2 = lock0.clone();

    // spawn ... スポーン
    let th0 = thread::spawn(move || {reader(lock0, 0)});
    let th1 = thread::spawn(move || {reader(lock1, 1)});
    let th2 = thread::spawn(move || {writer(lock2)});

    th0.join().unwrap();
    th1.join().unwrap();
    th2.join().unwrap();
}
