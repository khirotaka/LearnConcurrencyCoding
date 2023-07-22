use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;
use rand::Rng;

fn worker(barrier: Arc<Barrier>, id: usize) {
    let sleep_time = rand::thread_rng().gen_range(1..5);
    println!("worker-{}: sleep {}s", id, sleep_time);
    thread::sleep(Duration::from_secs(sleep_time));
    println!("worker-{}: スリープ終了", id);
    barrier.wait();     // 待ち合わせする。各スレッドは他のすべてのスレッドがここに到達するまで待機。全員揃ったら次に進む
    println!("worker-{}: AFTER barrier", id);
}

fn main() {
    let n_threads = 10;
    let mut v = Vec::new();
    let barrier = Arc::new(Barrier::new(n_threads));

    for i in 0..n_threads {
        let b = barrier.clone();
        let th = thread::spawn(move || {
            worker(b, i);
        });
        v.push(th);
    }

    for th in v {
        th.join().unwrap();
    }
}