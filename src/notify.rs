use core::future::{Future, IntoFuture};

use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::signal::Signal;

pub struct Notify(Signal<ThreadModeRawMutex, ()>);

/// Synchronization primitive for communication between futures,
/// where no meaningful data has to be transferred.
///
/// Used to wake up other futures as a replacement for callbacks in traditional UI libraries.
impl Notify {
    pub fn new() -> Self {
        Self(Signal::new())
    }

    pub async fn wait(&self) {
        let () = self.0.wait().await;
        self.0.reset();
    }

    /// Handle a single notification from this [Notify].
    ///
    /// If you need to handle notifications in a loop, consider using [Notify::on].
    pub async fn once<C>(&self, on_notify: C) -> C::Output
    where
        C: IntoFuture,
    {
        self.wait().await;
        on_notify.await
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
        F: Future<Output = R>,
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

    pub fn notify(&self) {
        self.0.signal(())
    }
}
