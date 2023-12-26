use std::thread;

fn main() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);
    println!("Hello, from the main thread!");

    t1.join().expect("failed to wait for thread t1");
    t2.join().expect("failed to wait for thread t2");

    let numbers = vec![1, 2, 3];
    thread::spawn(move || {
        let id = thread::current().id();
        for n in numbers {
            println!("{:?}: {}", id, n);
        }
    })
    .join()
    .expect("failed to join thread t3");
}

fn f() {
    println!("Hello from another thread!");
    println!("My Thread Id is: {:?}", thread::current().id())
}
