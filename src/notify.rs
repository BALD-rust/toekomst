use core::future::IntoFuture;

use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::signal::Signal;

/// Synchronization primitive for communication between futures.
///
/// Used to wake up other futures as a replacement for callbacks in traditional UI libraries.
pub struct Notify<T = ()>(Signal<ThreadModeRawMutex, T>);

impl<T: Send> Notify<T> {
    pub const fn new() -> Self {
        Self(Signal::new())
    }

    pub fn new_preoccupied(v: T) -> Self {
        let s = Self::new();
        s.notify(v);
        s
    }

    pub async fn wait(&self) -> T {
        let t = self.0.wait().await;
        self.0.reset();
        t
    }

    /// Handle a single notification from this [Notify].
    ///
    /// If you need to handle notifications in a loop, consider using [Notify::on].
    pub async fn once<C, F>(&self, on_notify: C) -> F::Output
    where
        F: IntoFuture,
        C: FnOnce(T) -> F,
    {
        let t = self.wait().await;
        on_notify(t).await
    }

    /// Create a handler for this [Notify].
    ///
    /// This function's arguments are rather complicated: because futures aren't `Clone`,
    /// we need to provide [Notify::on] with a function that *creates* the handler future when a notification
    /// is received. Typically, this will be in the form of a `Clone` closure that creates an `async`
    /// block.
    ///
    /// Example usage:
    ///
    /// ```rust
    /// let n = Notify::new();
    ///
    /// let fut = n.on(|| async { println!("Notification received!") });
    /// fut.await;
    /// ```
    pub async fn on<C, F, R>(&self, handler: C) -> !
    where
        F: IntoFuture<Output = R>,
        C: (Fn() -> F) + Clone,
    {
        loop {
            self.wait().await;
            // handler is a method that creates futures
            let fut_gen = handler.clone();
            // fut_gen() creates the future that handles this notification
            let fut = fut_gen();
            // handle the notification
            fut.await;
        }
    }

    pub fn notify(&self, val: T) {
        self.0.signal(val)
    }
}
