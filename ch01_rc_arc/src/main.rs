use std::{sync::Arc, thread};

fn main() {

    let a = Arc::new([1, 2, 3]);
    let b = a.clone();
    println!("address of a: {:p}", &a);
    println!("address of b: {:p}", &b);
    println!("address pointed to by a: {:p}", a);
    println!("address pointed to by b: {:p}", b);

    thread::spawn(move || { dbg!(a)}).join().unwrap();
    thread::spawn(move || { dbg!(b)}).join().unwrap();
    println!("Hello, world!");
}
