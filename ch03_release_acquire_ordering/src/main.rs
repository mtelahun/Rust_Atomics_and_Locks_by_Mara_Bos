use std::{sync::atomic::{AtomicU64, AtomicBool, Ordering}, thread, time::Duration};

static DATA: AtomicU64 = AtomicU64::new(0);
static READY: AtomicBool = AtomicBool::new(false);

fn main() {
    thread::spawn(|| {
        DATA.store(123, Ordering::Relaxed);
        READY.store(true, Ordering::Release);
    });
    while !READY.load(Ordering::Acquire) {
        thread::sleep(Duration::from_millis(100));
        println!("Waiting...");
    }
    println!("{}", DATA.load(Ordering::Relaxed));
}
