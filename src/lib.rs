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
use std::pin::Pin;
use std::sync::Arc;
use std::sync::Condvar;
use std::sync::Mutex;
use std::sync::PoisonError;
use std::task::Context;
use std::task::Poll;
use std::task::Wake;
use std::task::Waker;

#[derive(Default)]
struct CondvarWake {
    park: Condvar,
    awoken: Mutex<bool>,
}

impl CondvarWake {
    pub fn park(&self) {
        let mut guard = self.awoken.lock().unwrap_or_else(PoisonError::into_inner);

        // Until we are awoken, we can park on the condvar. This also handles the case where we're
        // awoken while we're actually polling.
        while !*guard {
            guard = self
                .park
                .wait(guard)
                .unwrap_or_else(PoisonError::into_inner);
        }

        *guard = false;
    }
}

impl Wake for CondvarWake {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref()
    }

    fn wake_by_ref(self: &Arc<Self>) {
        *self.awoken.lock().unwrap_or_else(PoisonError::into_inner) = true;
        self.park.notify_one();
    }
}

/// Block on specified [`Future`].
///
/// The future will be polled until completion on the current thread.
pub fn execute<T>(f: impl Future<Output = T>) -> T {
    // Use dynamic dispatch to save on codegen
    poll(std::pin::pin!(f))
}

/// Poll a future until completion.
fn poll<T>(mut pinned: Pin<&mut dyn Future<Output = T>>) -> T {
    let wake = Arc::new(CondvarWake::default());
    let waker = Waker::from(Arc::clone(&wake));

    let mut context = Context::from_waker(&waker);

    loop {
        match pinned.as_mut().poll(&mut context) {
            Poll::Ready(value) => return value,
            Poll::Pending => wake.park(),
        }
    }
}
