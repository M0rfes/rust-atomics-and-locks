use std::thread;

pub fn run() {
    let tq = thread::spawn(f);
    let t2 = thread::spawn(f);
    println!("Hello from the main thread.");
    tq.join();
    t2.join();
}

fn f() {
    println!("Hello from thread");
    let id = thread::current().id();
    println!("my id is {id:?}");
}
