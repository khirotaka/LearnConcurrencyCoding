use std::sync::{Condvar, Mutex};

pub struct Semaphore {
    mutex: Mutex<isize>,
    cond: Condvar,
    max: isize,
}

impl Semaphore {
    pub fn new(max: isize) -> Semaphore {
        Semaphore {
            mutex: Mutex::new(0),
            cond: Condvar::new(), // 条件変数
            max,                  // 同時にロック可能なプロセス数
        }
    }

    pub fn wait(&self) {
        // 現在進行中のスレッドの数を示す cnt
        let mut cnt = self.mutex.lock().unwrap();
        // ロックしてカウントが指定した最大値以上なら
        while *cnt >= self.max {
            cnt = self.cond.wait(cnt).unwrap(); // 条件変数のwaitで待機
                                                // ある条件が満たされない間はプロセスを待機。
                                                // .wait() は MutexGuardを引数にとり、それがロックを解放するまで待機する。
        }
        *cnt += 1; // カウントをインクリメントしてクリティカルセッションへ遷移
    }

    pub fn post(&self) {
        let mut cnt = self.mutex.lock().unwrap();
        *cnt -= 1;
        if *cnt <= self.max {
            self.cond.notify_one();
            // 条件変数にブロックされているスレッドのうち、一つを再開させる。
        }
    }
}
