use std::thread;
fn main() {
    thread::spawn(|| println!("hello Ira!")).join().unwrap();
    thread::spawn(|| println!("hello Suman!")).join().unwrap();
}

