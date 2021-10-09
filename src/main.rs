use std::{thread, sync::atomic::Ordering};

fn main() {
    thread::spawn(|| println!("hello Ira!")).join().unwrap();
    thread::spawn(|| println!("hello Suman!")).join().unwrap();
}

