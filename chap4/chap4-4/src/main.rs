// デットロックになるRwLockの例 2

use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let val = Arc::new(RwLock::new(true));
    let t = thread::spawn(move || {
        let _flag = val.read().unwrap();
        // _flag: RwLockReadGuard<bool>
        // _flag はスコープを抜けるまでReadロックを保持している。
        println!("deadlock");
        *val.write().unwrap() = false;
        // writeでロックを獲得しようとするが、_flag がスコープを抜けるまでロックを獲得しているため、操作できない
    });

    t.join().unwrap();
}
