#[cfg(not(feature = "async_std"))]
mod imp {
    use std::sync::mpsc;

    pub struct Sender<T>(mpsc::SyncSender<T>);

    impl<T> Sender<T> {
        #[must_use]
        pub fn send_blocking(&self, item: T) -> bool {
            self.0.send(item).is_ok()
        }
    }

    /// Receiving end of channel.
    pub type Receiver<T> = mpsc::Receiver<T>;

    pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
        let (s, r) = mpsc::sync_channel(0);
        (Sender(s), r)
    }
}

#[cfg(feature = "async_std")]
mod imp {
    use async_std::channel;

    pub struct Sender<T>(channel::Sender<T>);

    impl<T> Sender<T> {
        #[must_use]
        pub fn send_blocking(&self, item: T) -> bool {
            async_std::task::block_on(async { self.0.send(item).await.is_ok() })
        }
    }

    /// Receiving end of channel.
    pub type Receiver<T> = channel::Receiver<T>;

    pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
        let (s, r) = channel::bounded(1);
        (Sender(s), r)
    }
}

pub use imp::*;
