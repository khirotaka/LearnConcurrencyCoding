mod hello;
use hello::Hello;

use futures::future::{BoxFuture, FutureExt};
use futures::task::{waker_ref, ArcWake};
use std::future::Future;
use std::sync::{Arc, Mutex};
use std::task::Context;

struct Task {
    hello: Mutex<BoxFuture<'static, ()>>,   // 'static は、プログラムが終了するまであたいはドロップされない
}

impl Task {
    fn new() -> Self {
        let hello = Hello::new();
        Task {
            hello: Mutex::new(hello.boxed()),
        }
    }
}

impl ArcWake for Task {
    fn wake_by_ref(_arc_self: &Arc<Self>) {}
}

// 状態を持つ関数
fn main() {
    let task = Arc::new(Task::new());
    let waker = waker_ref((&task));
    let mut ctx = Context::from_waker(&waker);
    let mut hello = task.hello.lock().unwrap();

    let _ = hello.as_mut().poll(&mut ctx);
    let _ = hello.as_mut().poll(&mut ctx);
    let _ = hello.as_mut().poll(&mut ctx);
}
