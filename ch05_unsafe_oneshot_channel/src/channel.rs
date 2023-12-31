use std::{cell::UnsafeCell, mem::MaybeUninit, sync::atomic::{AtomicBool, Ordering}};

#[derive(Debug)]
pub struct Channel<T> {
    ready: AtomicBool,
    message: UnsafeCell<MaybeUninit<T>>,
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Self {
            ready: AtomicBool::new(false),
            message: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }

    pub unsafe fn send(&self, message: T) {
        (*self.message.get()).write(message);
        self.ready.store(true, Ordering::Release)
    }

    pub fn is_ready(&self) -> bool {
        self.ready.load(Ordering::Acquire)
    }

    pub unsafe fn receive(&self) -> T {
        (*self.message.get()).assume_init_read()
    }
}