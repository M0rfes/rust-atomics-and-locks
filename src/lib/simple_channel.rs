use std::{
    collections::VecDeque,
    sync::{Condvar, Mutex},
};

struct Channel<T> {
    queue: Mutex<VecDeque<T>>,
    condvar: Condvar,
}

impl<T> Channel<T> {
    fn new() -> Self {
        Channel {
            queue: Mutex::new(VecDeque::new()),
            condvar: Condvar::new(),
        }
    }

    fn send(&self, message: T) {
        self.queue.lock().unwrap().push_back(message);
        self.condvar.notify_all();
    }

    fn receve(&self) -> T {
        let mut q = self.queue.lock().unwrap();
        loop {
            if let Some(message) = q.pop_front() {
                return message;
            } else {
                q = self.condvar.wait(q).unwrap();
            }
        }
    }
}
