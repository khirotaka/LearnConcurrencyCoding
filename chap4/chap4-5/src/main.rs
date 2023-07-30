use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let val = Arc::new(RwLock::new(true));

    let t = thread::spawn(move || {
        let _unused = val.read().unwrap();
        // let _ = val.read().unwrap(); はコンパイルエラー (v1.71.0)
        drop(_unused);  // dropして破棄すれば、ロックが解放される。
        *val.write().unwrap() = false;
        println!("not deadlock");
    });

    t.join().unwrap();
}