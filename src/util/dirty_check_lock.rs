use std::sync::atomic::AtomicBool;

use parking_lot::RwLock;

/// A reader-writer lock with dirty state check. The dirty state will be set whenever
/// the lock is write-locked, and is cleared manually or at reads.
#[derive(Debug)]
pub struct DirtyCheckLock<T> {
    lock: RwLock<T>,
    dirty: AtomicBool,
}

pub type ReadGuard<'r, T> = parking_lot::lock_api::RwLockReadGuard<'r, parking_lot::RawRwLock, T>;
pub type WriteGuard<'w, T> = parking_lot::lock_api::RwLockWriteGuard<'w, parking_lot::RawRwLock, T>;

impl<T> DirtyCheckLock<T> {
    /// Create a new instance with the given value. Sets `dirty` to `true` when created.
    pub fn new(t: T) -> DirtyCheckLock<T> {
        DirtyCheckLock {
            lock: RwLock::new(t),
            dirty: AtomicBool::new(true),
        }
    }

    /// Create a new instance from the given lock. Sets `dirty` to `true` when created.
    pub fn from_lock(lock: RwLock<T>) -> DirtyCheckLock<T> {
        DirtyCheckLock {
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

    /// Access the inner contents without setting the dirty flag. The content
    /// is readonly in this case.
    pub fn read(&self, clear_dirty: bool) -> ReadGuard<'_, T> {
        let lock = self.lock.read();
        if clear_dirty {
            self.clear_dirty()
        }
        lock
    }

    /// Access the inner contents, and set the dirty flag to true.
    pub fn write(&self) -> WriteGuard<'_, T> {
        let res = self.lock.write();
        self.set_dirty(true);
        res
    }
}
