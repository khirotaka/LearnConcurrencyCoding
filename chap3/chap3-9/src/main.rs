mod semaphore;
// セマフォ

use semaphore::Semaphore;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

const NUM_LOOP: usize = 100_000;
const NUM_THREADS: usize = 8;
const SEM_NUM: isize = 4;

// usize型をアトミック(スレッドセーフ) に扱うための型
static mut CNT: AtomicUsize = AtomicUsize::new(0);

fn main() {
    let mut v = Vec::new();
    let sem = Arc::new(Semaphore::new(SEM_NUM));

    for i in 0..NUM_THREADS {
        let s = sem.clone();
        let t = std::thread::spawn(move || {
            for _ in 0..NUM_LOOP {
                // ロックを獲得できるまで待機
                s.wait();

                unsafe {
                    // CNTをアトミックにインクリメント
                    CNT.fetch_add(1, Ordering::SeqCst);
                };

                // CNT の値をアトミックに読み出す。結果は n に代入される。
                let n = unsafe {
                    // SeqCst ... Sequentially Consistent Ordering(シーケンス一貫性オーダリング)
                    // Memory Orderingは操作の順序に対する補償の強さを表現する。
                    // https://zenn.dev/belle/articles/bcddf554a43053
                    // コンパイラは指定されたMemory Orderingの保証を満たす限りはプログラムの意味を変えない範囲で
                    // 命令を並べ替えて良い。
                    // SeqCst は最も厳格なMemory Orderingで
                    // 全てのスレッドが同一の順序で全ての操作を見ることを保証する。
                    // これにより、各スレッドはお互いに一貫した順序で操作を見ることができ、
                    // 意図しないデータ競合を防ぐことができる。

                    // memo
                    // C言語のvolatileとRustのOrderingは、メモリアクセスの観点からは関連性があるように見えるが、役割と目的は異なる。
                    // volatileはコンパイラの最適化を抑制するのに対して、Orderingはスレッド間のメモリアクセスの順序を制御し、
                    // 同期を実現する。
                    CNT.load(Ordering::SeqCst)
                };

                println!("semaphore: i = {}, CNT = {}", i, n);
                assert!((n as isize) <= SEM_NUM);
                unsafe {
                    // CNTをアトミックにデクリメント
                    CNT.fetch_sub(1, Ordering::SeqCst)
                };
                s.post();   // ロック解放
            }
        });
        v.push(t);
    }

    for t in v {
        t.join().unwrap();
    }
}
