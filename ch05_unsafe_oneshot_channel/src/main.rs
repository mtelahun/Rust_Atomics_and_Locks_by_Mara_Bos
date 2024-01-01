use std::thread;

use ch05_unsafe_oneshot_channel::channel::Channel;

fn main() {
    let mut channel = Channel::new();
    thread::scope(|s| {
        let (sender, reciever) = channel.split();
        s.spawn(|| {
            sender.send("hello world!");
        });

        assert_eq!(reciever.receive(), "hello world!");
    });
}
