use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub enum StateHello {
    HELLO,
    WORLD,
    END,
}

pub struct Hello {
    state: StateHello,
}

impl Hello {
    pub fn new() -> Self {
        Hello {
            state: StateHello::HELLO,
        }
    }
}

impl Future for Hello {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.state {
            StateHello::HELLO => {
                println!("Hello, ");
                // WORLD状態に遷移
                self.state = StateHello::WORLD;
                Poll::Pending
            }
            StateHello::WORLD => {
                println!("World!");
                // END状態に遷移
                self.state = StateHello::WORLD;
                Poll::Pending
            }
            StateHello::END => Poll::Ready(()),
        }
    }
}
