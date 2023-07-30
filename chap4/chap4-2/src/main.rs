// RwLock(デットロック)

use std::sync::{Arc, RwLock};
use std::thread;

// スレッドは、メインスレッドと、thread::spawnで生成されるスレッドの２個
fn main() {
    // RwLockで読み込み: 同時アクセス可 書き込み: 排他的
    let val = Arc::new(RwLock::new(true));
    // スレッド生成、moveで valの所有権が遷移する
    let t = thread::spawn(move || {
        // Readロックは、複数のReadロックは許可する
        let flag = val.read().unwrap(); // Readロックを獲得
        // val = trueなら
        if *flag {
            // Writeロックは排他的なので、ロック獲得中はReadもWriteも許されない。
            *val.write().unwrap() = false;  // Readロック中にWriteロックを獲得。
            println!("flag is true");
        }
    });

    t.join().unwrap();
}