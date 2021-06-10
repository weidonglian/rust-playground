use std::time::{Instant, Duration};
use std::future::Future;
use std::task::{Context, Poll};
use std::pin::Pin;

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            println!("Delay is triggering...");
            Poll::Ready("done")
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[tokio::test]
async fn delay_future() {
    let start = Instant::now();
    let when = start + Duration::from_millis(10);
    let out = Delay { when }.await;
    println!("Time taken: {:?} with {}", Instant::now() - start, out);
    assert!(Instant::now() >= when);
}