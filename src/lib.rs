//! Beul - It executes futures.
//!
//! This crate offers a single function: [`execute`], which will run a single future on the current
//! thread. No fancy executor, no future primitives, just a simple executor. No dependencies, no
//! unsafe rust.
//!
//! The design is based on the example `ThreadWaker` from the [`Wake`] documentation, which the
//! reentrancy issues resolved by using a [`Condvar`].
//!
//! Beul is Dutch for executioner.
//!
//! # Usage
//!
//! ```
//! beul::execute(async {});
//! ```
#![forbid(unsafe_code)]
use std::future::Future;
use std::sync::Arc;
use std::sync::Condvar;
use std::sync::Mutex;
use std::sync::PoisonError;
use std::task::Context;
use std::task::Poll;
use std::task::Wake;
use std::task::Waker;

#[derive(Default)]
struct CondvarWake(Condvar);

impl Wake for CondvarWake {
    fn wake(self: Arc<Self>) {
        self.0.notify_one()
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.0.notify_one()
    }
}

/// Block on specified [`Future`].
pub fn execute<T>(f: impl Future<Output = T>) -> T {
    // TODO: replace with std::pin::pin once it gets stabilized
    let mut pinned = Box::pin(f);

    let wake = Arc::new(CondvarWake::default());
    let waker = Waker::from(Arc::clone(&wake));

    let mut context = Context::from_waker(&waker);

    let mutex = Mutex::new(());
    // Cannot panic but avoids generating the unwrap code
    let mut guard = mutex.lock().unwrap_or_else(PoisonError::into_inner);

    loop {
        match pinned.as_mut().poll(&mut context) {
            Poll::Ready(value) => return value,
            Poll::Pending => guard = wake.0.wait(guard).unwrap_or_else(PoisonError::into_inner),
        }
    }
}
