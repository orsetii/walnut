/// Synchronization interfaces.
pub mod interface {

    /// Any object implementing this trait guarantees exclusive access to the data wrapped within
    /// the Mutex for the duration of the provided closure.
    pub trait Mutex {
        /// The type of the data that is wrapped by this mutex.
        type Data;

        /// Locks the mutex and grants the closure temporary mutable access to the wrapped data.
        fn lock<R>(&self, f: impl FnOnce(&mut Self::Data) -> R) -> R;
    }
}
pub mod mutex;

pub use mutex::NullLock;