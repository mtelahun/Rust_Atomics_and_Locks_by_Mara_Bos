use std::thread;

use ch05_unsafe_oneshot_channel::channel::Channel;

fn main() {
    let mut channel = Channel::new();
    let t = thread::current();
    thread::scope(|s| {
        let (sender, reciever) = channel.split();
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
