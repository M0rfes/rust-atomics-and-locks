use std::sync::Mutex;
use std::thread;

pub fn run() {
    let n = Mutex::new(0);

    thread::scope(|s| {
        for _ in 0..100 {
            s.spawn(|| {
                let mut lock = n.lock().unwrap();
                for _ in 1..10 {
                    *lock += 1;
                }
            });
        }
    });
    let sum = n.into_inner().unwrap();
    println!("{sum}");
}
