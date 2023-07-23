use std::ptr::{read_volatile, write_volatile}; // コンパイラによる最適化を抑制してメモリ読み書きをする関数
use std::sync::atomic::{fence, Ordering}; // メモリバリア用の関数
use std::thread;

const NUM_THREADS: usize = 4;
const NUM_LOOP: usize = 100_000;

/// volatile用のマクロ
/// macro_rules! は
/// デリミティブマクロ(メタプログラミングの一種)と呼ばれるものを生成する。
/// 繰り返しやパターンマッチングに基づくコード生成のために使用する
/// 構文は
/// macro_rules! macro_name {
///     (pattern) => {
///         // コード
///     };
/// }
/// $addr: expr の$addr は変数、expr は変数のタイプ
/// expr は式を意味し、整数、文字列、関数呼び出し、算術式など、計算結果を返す任意のコードを指すことができる。
macro_rules! read_mem {
    ($addr: expr) => {
        unsafe { read_volatile($addr) }
    };
}

macro_rules! write_mem {
    ($addr: expr, $val: expr) => {
        unsafe { write_volatile($addr, $val) }
    };
}

// ロック管理用の型
struct LockGuard {
    idx: usize,
}

// Dropトレイト ... オブジェクトがスコープを抜ける時に実行される。C++のデストラクタと同じ概念
impl Drop for LockGuard {
    // ロック解放処理
    fn drop(&mut self) {
        fence(Ordering::SeqCst);
        write_mem!(&mut LOCK.tickets[self.idx], None);
    }
}

struct BakeryLock {
    entering: [bool; NUM_THREADS],
    tickets: [Option<u64>; NUM_THREADS],
}

impl BakeryLock {
    // ロック関数 idx はスレッド番号
    fn lock(&mut self, idx: usize) -> LockGuard {
        // メモリバリアを行い、アウトオブオーダーでのメモリ読み書きをさせないようにしている
        fence(Ordering::SeqCst);
        // スレッドidx番がチケット取得中状態であることを示すために、entering[idx] に trueを設定
        write_mem!(&mut self.entering[idx], true);
        // メモリバリアを行い、アウトオブオーダーでのメモリ読み書きをさせないようにしている
        fence(Ordering::SeqCst);

        // 現在配布されているチケットの最大値を取得
        let mut max = 0;
        for i in 0..NUM_THREADS {
            // read_mem! の結果が None でないなら、 Some(t) にはいる
            // t と max の値を比較し、大きい方を変数 max にする
            if let Some(t) = read_mem!(&self.tickets[i]) {
                max = max.max(t);
            }
        }

        let ticket = max + 1; // 最大値 + 1 を自分のチケット番号にする
        write_mem!(&mut self.tickets[idx], Some(ticket)); // チケット番号を書き込む

        // メモリバリアを行い、アウトオブオーダーでのメモリ読み書きをさせないようにしている
        fence(Ordering::SeqCst);
        // チケットを取得したことを示すために false を設定
        write_mem!(&mut self.entering[idx], false);
        // メモリバリアを行い、アウトオブオーダーでのメモリ読み書きをさせないようにしている
        fence(Ordering::SeqCst);

        // 待機処理開始
        // 自分より若い番号を持ったスレッドがいる場合に待機
        for i in 0..NUM_THREADS {
            if i == idx {
                continue;
            }

            // スレッド i がチケット取得中なら待機
            while read_mem!(&self.entering[i]) {}

            loop {
                // チケット番号を読み込み
                match read_mem!(&self.tickets[i]) {
                    Some(t) => {
                        // ticket(自分のチケット番号) と 読み出したスレッドi番目のチケット番後を比較
                        // 自分のチケット番号が小さい or チケット番号が同じで且つ自分のスレッド番号が小さいなら待機終了
                        if ticket < t || (ticket == t && idx < i) {
                            break;
                        }
                    }
                    None => {
                        // スレッドiが首里中でないなら待機終了
                        break;
                    }
                }
            }
        }

        fence(Ordering::SeqCst);
        LockGuard { idx }
    }
}

static mut LOCK: BakeryLock = BakeryLock {
    entering: [false; NUM_THREADS],
    tickets: [None; NUM_THREADS],
};

static mut COUNT: u64 = 0;

fn main() {
    let mut v = Vec::new();
    for i in 0..NUM_THREADS {
        let th = thread::spawn(move || {
            for _ in 0..NUM_LOOP {
                // ロック獲得
                let _lock = unsafe { LOCK.lock(i) };
                unsafe {
                    let c = read_volatile(&COUNT);
                    write_volatile(&mut COUNT, c + 1);
                }
            }
        });
        v.push(th);
    }

    for th in v {
        th.join().unwrap();
    }

    println!(
        "COUNT = {} (expected = {})",
        unsafe { COUNT },
        NUM_LOOP * NUM_THREADS
    )
}
