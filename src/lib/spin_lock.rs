use std::{
    cell::UnsafeCell,
    hint::spin_loop,
    ops::{Deref, DerefMut},
    sync::atomic::AtomicBool,
    thread,
};

struct SpinLock<T> {
    is_locked: AtomicBool,
    value: UnsafeCell<T>,
}
unsafe impl<T> Sync for SpinLock<T> {}

impl<T> SpinLock<T> {
    pub fn new(value: T) -> Self {
        SpinLock {
            value: UnsafeCell::new(value),
            is_locked: AtomicBool::new(false),
        }
    }
    pub fn lock(&self) -> SpinGuard<T> {
        while self
            .is_locked
            .swap(true, std::sync::atomic::Ordering::Acquire)
        {
            spin_loop();
        }
        SpinGuard { lock: self }
    }
}

struct SpinGuard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<T> Deref for SpinGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for SpinGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<T> Drop for SpinGuard<'_, T> {
    fn drop(&mut self) {
        self.lock
            .is_locked
            .store(false, std::sync::atomic::Ordering::Release);
    }
}

pub fn run() {
    let x = SpinLock::new(Vec::new());
    thread::scope(|s| {
        s.spawn(|| x.lock().push(1));
        s.spawn(|| {
            let mut g = x.lock();
            g.push(2);
            g.push(2);
        });
    });
    let g = x.lock();
    assert!(g.as_slice() == [1, 2, 2] || g.as_slice() == [2, 2, 1]);
}
