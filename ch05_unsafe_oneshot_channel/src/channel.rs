use std::{cell::UnsafeCell, mem::MaybeUninit, sync::{atomic::{AtomicBool, Ordering}, Arc}};

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let a = Arc::new(Channel::<T>::new());
    (Sender { channel: a.clone() }, Receiver { channel: a })
}

pub struct Sender<T> {
    channel: Arc<Channel<T>>,
}

pub struct Receiver<T> {
    channel: Arc<Channel<T>>,
}

impl<T> Sender<T> {
    pub fn send(self, message: T) {
        unsafe { (*self.channel.message.get()).write(message) };
        self.channel.ready.store(true, Ordering::Release)
    }
}

impl<T> Receiver<T> {

    pub fn is_ready(&self) -> bool {
        self.channel.ready.load(Ordering::Relaxed)
    }

    /// Panics if no message is available yet or if the message
    /// was already consumed.
    /// 
    /// Tip: use `is_ready` to check first.
    pub fn receive(self) -> T {
        if !self.channel.ready.swap(false, Ordering::Acquire) {
            panic!("no message available!");
        }

        // Safety: we've just checked (and reset) the ready flag
        unsafe { (*self.channel.message.get()).assume_init_read() }
    }
}

#[derive(Debug)]
struct Channel<T> {
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
}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe { self.message.get_mut().assume_init_drop() }
        }
    }
}
