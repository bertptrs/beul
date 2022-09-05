use std::future::Future;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::task::Context;
use std::task::Poll;
use std::thread;

#[test]
fn test_simple() {
    // Sanity check
    assert_eq!(42, beul::execute(async { 42 }));

    // Some more esoteric futures
    beul::execute(async { async {}.await });
    beul::execute(async { beul::execute(async {}) });
}

#[test]
fn test_threaded_future() {
    /// Dummy future that sleeps until a separate thread wakes it
    ///
    /// It returns the number of times it has been awoken
    struct ThreadedFuture(Arc<AtomicBool>, usize);

    impl Future for ThreadedFuture {
        type Output = usize;

        fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            // Note down that we've been polled
            self.1 += 1;

            if self.0.load(Ordering::Acquire) {
                Poll::Ready(self.1)
            } else {
                if self.1 == 1 {
                    let completer = Arc::clone(&self.0);
                    let waker = cx.waker().clone();
                    thread::spawn(move || {
                        completer.store(true, Ordering::Release);
                        waker.wake();
                    });
                }

                Poll::Pending
            }
        }
    }

    let future = ThreadedFuture(Arc::new(AtomicBool::new(false)), 0);

    // Future should be polled twice, once initially and once after the wake-up
    assert_eq!(beul::execute(future), 2);
}

#[test]
fn test_self_waking_futures() {
    struct SelfWakingFuture(bool);

    impl Future for SelfWakingFuture {
        type Output = ();

        fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            if self.0 {
                Poll::Ready(())
            } else {
                // Next time we complete
                self.0 = true;
                // Request to be woken up
                cx.waker().wake_by_ref();

                Poll::Pending
            }
        }
    }

    beul::execute(SelfWakingFuture(false));
}
