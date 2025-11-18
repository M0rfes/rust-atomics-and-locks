use std::thread;

pub fn run() {
    let tq = thread::spawn(f);
    let t2 = thread::spawn(f);
    println!("Hello from the main thread.");
    tq.join().unwrap();
    t2.join().unwrap();
}

fn f() {
    println!("Hello from thread");
    let id = thread::current().id();
    println!("my id is {id:?}");
}
