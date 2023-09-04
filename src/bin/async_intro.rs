// Async functions in Rust need to implement std::future::Future trait
// when calling this function thats async we get a handler that we can await.
// Rust future does not represent a computation happening in the background, 
// rather its the computation itself.

// Rust future are **state machines**

// Futures must have `poll` called on them to advance their state.
// Futures can be composed of other futures.

// Tokio executor is responsible for calling `poll` on the outmost function
use std::pin::Pin;
use std::future::Future;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

struct Delay {
    when: Instant
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            println!("Hello world");
            Poll::Ready("done")
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
        
    }
}

enum MainFuture {
    // Initialized, never polled
    State0,
    // Waiting on `Delay`, i.e. the `future.await` line.
    State1(Delay),
    // The future has completed.
    Terminated,
}

impl Future for MainFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            match *self {
                MainFuture::State0 => {
                    let when = Instant::now() + Duration::from_millis(10);
                    let future = Delay { when };
                    *self = Self::State1(future)
                }
                MainFuture::State1(ref mut my_future) => {
                    match Pin::new(my_future).poll(cx) {
                        Poll::Ready(out) => {
                            assert_eq!(out, "done");
                            *self = MainFuture::Terminated;
                            return Poll::Ready(());
                        }
                        Poll::Pending => {
                            return Poll::Pending;
                        }
                    }
                }
                MainFuture::Terminated => {
                    panic!("future polled after completion")
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let when = Instant::now() + Duration::from_millis(10);
    let future = Delay { when };

    let out = future.await;
    assert_eq!(out, "done");
}
