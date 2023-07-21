// 条件変数
use std::sync::{Arc, Mutex, Condvar};
use std::thread;

// Condvar型の変数が条件変数

fn child(id: u64, p: Arc<(Mutex<bool>, Condvar)>) {
    println!("start child: {}", id);
    let &(ref lock, ref cvar) = &*p;

    let mut started = lock.lock().unwrap();
    // 共有変数 started が false の間ループする
    // whileを使う以外に、 cvar.wait_while(started, |started| !*started).unwrap() でも可
    while !*started {
        started = cvar.wait(started).unwrap();
    }

    // 共有変数 startedが trueになる whileを抜けて初めて println される
    println!("child: {}", id);
}

fn parent(p: Arc<(Mutex<bool>, Condvar)>) {
    // parentがロックを獲得して 条件変数 を trueにしない限り
    // childは動作しない
    let &(ref lock, ref cvar) = &*p;

    let mut started = lock.lock().unwrap();     // lockを獲得
    *started = true;    // startの値をtrueに書き換え
    cvar.notify_all();  // 状態変数が trueになったことを通知
    println!("parent");
}

fn main() {
    let pair0 = Arc::new((Mutex::new(false), Condvar::new()));
    let pair1 = pair0.clone();
    let pair2 = pair0.clone();

    let c0 = thread::spawn(move || { child(0, pair0) });
    let c1 = thread::spawn(move || { child(1, pair1) });
    let c2 = thread::spawn(move || { parent(pair2) });

    c0.join().unwrap();
    c1.join().unwrap();
    c2.join().unwrap();
}
