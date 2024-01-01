use std::thread;

use ch05_unsafe_oneshot_channel::channel::channel;

fn main() {
    let (sender, reciever) = channel();
    let t = thread::current();
    thread::scope(|s| {
        s.spawn(|| {
            sender.send("hello world!");
            t.unpark()
        });
        while !reciever.is_ready() {
            thread::park();
        }

        assert_eq!(reciever.receive(), "hello world!");
    });
}
