use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let val = Arc::new(RwLock::new(true));

    let t = thread::spawn(move || {
        let flag = *val.read().unwrap();    // この行の終わりで、RwLockReadGuard がDropされるので、ロック解除
        // *(val.read().unwrap()) でも同じ意味。
        // *val と勘違いしてしまうが、正確には val.read().unwrap() で生まれる RwLockReadGuardオブジェクトを参照外ししている。
        // flagはbool型
        if flag {
            *val.write().unwrap() = false;
            println!("flag is true");
        }
    });

    t.join().unwrap();
}
