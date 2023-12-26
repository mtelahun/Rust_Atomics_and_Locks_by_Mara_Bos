use std::{
    sync::atomic::{AtomicU64, AtomicUsize, Ordering},
    thread,
    time::{Duration, Instant},
};

fn main() {
    let main_thread = thread::current();
    let num_done = &AtomicUsize::new(0);
    let total_time = &AtomicU64::new(0);
    let max_time = &AtomicU64::new(0);

    thread::scope(|s| {
        for t in 0..4 {
            let main_thread = main_thread.clone();
            s.spawn(move || {
                for i in 0..25 {
                    let start = Instant::now();
                    process_item((t + 1) * 20000 + i);
                    let time_taken = start.elapsed().as_micros() as u64;
                    num_done.fetch_add(1, Ordering::Relaxed);
                    total_time.fetch_add(time_taken, Ordering::Relaxed);
                    max_time.fetch_max(time_taken, Ordering::Relaxed);
                }
                main_thread.unpark();
            });
        }

        loop {
            let total_time = Duration::from_micros(total_time.load(Ordering::Relaxed));
            let max_time = Duration::from_micros(max_time.load(Ordering::Relaxed));
            let n = num_done.load(Ordering::Relaxed);
            if n == 100 {
                break;
            }
            if n == 0 {
                println!("Working... nothing done yet");
            } else {
                println!(
                    "Working... {n}/100 done, {:?} average, {:?} peak",
                    total_time / n as u32,
                    max_time
                );
            }
            thread::park_timeout(Duration::from_secs(1));
        }
    });
    println!("Finished {:?}/100", num_done);
}

fn process_item(arg: i32) {
    let mut _a = 0;
    for i in 0..arg {
        _a = i;
    }
}
