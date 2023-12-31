use std::thread;

use ch05_unsafe_oneshot_channel::channel::Channel;

fn main() {
    let channel = Channel::new();
    let t = thread::current();
    thread::scope(|s| {
        s.spawn(|| {
            channel.send("hello world!");
            t.unpark()
        });
        while !channel.is_ready() {
            thread::park();
        }

        assert_eq!(channel.receive(), "hello world!");
    });
}
