use core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering},
};

pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

/// we dont need to require that `T` is `Sync` because our `Guard<T>`
/// will only allow one thread at a time to access `T`
unsafe impl<T> Sync for Guard<'_, T> where T: Sync {}

pub struct SpinLock<T> {
    locked: AtomicBool,
    val: UnsafeCell<T>,
}

/// we dont need to require that `T` is `Sync` because our `SpinLock<T>`
/// will only allow one thread at a time to access `T`
unsafe impl<T> Sync for SpinLock<T> where T: Send {}

impl<T> SpinLock<T> {
    pub const fn new(val: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            val: UnsafeCell::new(val),
        }
    }

    /// Retreive a reference to the `T` value,
    /// locking the `SpinLock`
    #[inline]
    pub fn lock(&self) -> Guard<T> {
        while self
            .locked
            .swap(true, core::sync::atomic::Ordering::Acquire)
        {
            core::hint::spin_loop();
        }

        Guard { lock: self }
    }
}

impl<T> core::ops::Deref for Guard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        // Safety: The very existence of this Guard
        // guarantees we've exclusively locked the lock.
        unsafe { &*self.lock.val.get() }
    }
}

impl<T> core::ops::DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        // Safety: The very existence of this Guard
        // guarantees we've exclusively locked the lock.
        unsafe { &mut *self.lock.val.get() }
    }
}

impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        let prev_val = self
            .lock
            .locked
            .swap(false, core::sync::atomic::Ordering::AcqRel);

        // Assert that we are unlocking a LOCKED mutex
        // if we are not, we panic. This IS a bug, and leaving
        // this to create unforeseen consequences down the line
        // only creates worse and harder-to-debug errors/bugs.
        assert!(prev_val);
    }
}

pub struct OnceCell<T> {
    initialized: AtomicBool,
    value: Option<T>,
}

impl<T> OnceCell<T> {
    pub const fn new() -> Self {
        OnceCell {
            initialized: AtomicBool::new(false),
            value: None,
        }
    }

    pub fn get_or_init(&mut self, f: impl FnOnce() -> T) -> &mut T {
        if !self.initialized.load(Ordering::Relaxed) {
            self.value = Some(f());
            self.initialized.store(true, Ordering::Relaxed);
        }
        self.value.as_mut().unwrap()
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized.load(Ordering::Relaxed) && self.value.is_some()
    }
}
