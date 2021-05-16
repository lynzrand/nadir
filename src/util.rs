use std::{
    ops::{Deref, DerefMut},
    sync::{atomic::AtomicBool, RwLock},
};

/// A reader-writer lock with dirty state check. The dirty state will be set whenever
/// the lock is write-locked, and cleared manually.
pub struct DirtyCheckRwLock<T> {
    lock: RwLock<T>,
    dirty: AtomicBool,
}

impl<T> DirtyCheckRwLock<T> {
    /// Create a new instance with the given value. Sets `dirty` to `true` when created.
    pub fn new(t: T) -> DirtyCheckRwLock<T> {
        DirtyCheckRwLock {
            lock: RwLock::new(t),
            dirty: AtomicBool::new(true),
        }
    }

    /// Create a new instance from the given lock. Sets `dirty` to `true` when created.
    pub fn from_lock(lock: RwLock<T>) -> DirtyCheckRwLock<T> {
        DirtyCheckRwLock {
            lock,
            dirty: AtomicBool::new(true),
        }
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty.load(std::sync::atomic::Ordering::Acquire)
    }

    pub fn clear_dirty(&self) {
        self.dirty
            .store(false, std::sync::atomic::Ordering::Release);
    }

    pub fn set_dirty(&self, dirty: bool) {
        self.dirty
            .store(dirty, std::sync::atomic::Ordering::Release);
    }

    pub fn write(
        &self,
    ) -> Result<
        std::sync::RwLockWriteGuard<T>,
        std::sync::PoisonError<std::sync::RwLockWriteGuard<T>>,
    > {
        let res = self.lock.write();
        self.set_dirty(true);
        res
    }

    pub fn try_write(
        &self,
    ) -> Result<
        std::sync::RwLockWriteGuard<T>,
        std::sync::TryLockError<std::sync::RwLockWriteGuard<T>>,
    > {
        let res = self.lock.try_write();
        if res.is_ok() {
            self.set_dirty(true);
        }
        res
    }
}

impl<T> Deref for DirtyCheckRwLock<T> {
    type Target = RwLock<T>;

    fn deref(&self) -> &Self::Target {
        &self.lock
    }
}

impl<T> DerefMut for DirtyCheckRwLock<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.lock
    }
}
