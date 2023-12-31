use std::thread;

use channel::Channel;

pub mod channel;

pub const fn create_channel<T>() -> Channel<T> {
    Channel::<T>::new()
}

pub fn spawn_receiver(recv: Channel<i32>) {

    let _ = thread::spawn(move || {
        let payload = recv.receive();
        println!("Received: {}", payload);
    })
    .join();
}
