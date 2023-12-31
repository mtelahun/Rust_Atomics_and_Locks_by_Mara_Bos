use ch05_simple_channel::{channel::Channel, create_channel, spawn_receiver};

fn main() {
    let ch: Channel<i32> = create_channel::<i32>();

    ch.send(10);
    spawn_receiver(ch);
}
