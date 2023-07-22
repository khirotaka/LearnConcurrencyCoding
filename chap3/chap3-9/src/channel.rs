use crate::semaphore::Semaphore;
use std::collections::LinkedList;
use std::sync::{Arc, Condvar, Mutex};

/// チャネルの送信端
#[derive(Clone)]
pub struct Sender<T> {
    sem: Arc<Semaphore>,
    buf: Arc<Mutex<LinkedList<T>>>, // キュー
    cond: Arc<Condvar>,             // 読み込み側の条件変数
}

/// Sendトレイトを実装している方のみがチャンネルを介して送受信可能
/// これによって送受信してはいけないデータをコンパイル時に検知することができる。
/// 送受信してはいけないデータ: Rc型
impl<T: Send> Sender<T> {
    pub fn send(&self, data: T) {
        self.sem.wait(); // キューが最大値に達したら待機
        let mut buf = self.buf.lock().unwrap();
        buf.push_back(data); // エンキュー
        self.cond.notify_one(); // 読み込み側に通知
    }
}

/// チャンネルの受信端
pub struct Receiver<T> {
    sem: Arc<Semaphore>,
    buf: Arc<Mutex<LinkedList<T>>>,
    cond: Arc<Condvar>, // 読み込み側の条件変数
}

impl<T> Receiver<T> {
    pub fn recv(&self) -> T {
        let mut buf = self.buf.lock().unwrap();
        loop {
            // キューから取り出し
            if let Some(data) = buf.pop_front() {
                // もし buf.pop_front() が Some(data) を返したなら、 data がこのスコープに入る
                self.sem.post(); // セマフォのpost関数を呼び、セマフォのカウンタを-1する
                                 // セマフォのカウントが減ることで、キューに空きが出来たことを通知する
                return data; // メソッドの戻り値として dataを返す
            }
            // もし buf が空だったなら、受信用の状態変数で待機する。
            buf = self.cond.wait(buf).unwrap();
        }
    }
}

pub fn channel<T>(max: isize) -> (Sender<T>, Receiver<T>) {
    assert!(max > 0);
    let sem = Arc::new(Semaphore::new(max));
    let buf = Arc::new(Mutex::new(LinkedList::new()));
    let cond = Arc::new(Condvar::new());
    let tx = Sender {
        sem: sem.clone(),
        buf: buf.clone(),
        cond: cond.clone(),
    };
    let rx = Receiver { sem, buf, cond };
    (tx, rx)
}
