use std::{
    cell::UnsafeCell,
    marker::PhantomData,
    mem::MaybeUninit,
    sync::atomic::AtomicBool,
    thread::{self, Thread},
};

struct Channel<T> {
    ready: AtomicBool,
    message: UnsafeCell<MaybeUninit<T>>,
}

struct Sender<'a, T> {
    channel: &'a Channel<T>,
    receving_thread: Thread,
}

struct Recever<'a, T> {
    channel: &'a Channel<T>,
    _no_send: PhantomData<*const ()>,
}

impl<T> Channel<T> {
    fn new() -> Self {
        Channel {
            ready: AtomicBool::new(false),
            message: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }

    fn split(&mut self) -> (Sender<'_, T>, Recever<'_, T>) {
        *self = Self::new();
        (
            Sender {
                channel: self,
                receving_thread: thread::current(),
            },
            Recever {
                channel: self,
                _no_send: PhantomData,
            },
        )
    }
}

impl<'a, T> Sender<'a, T> {
    fn send(&self, value: T) {
        unsafe {
            (*self.channel.message.get()).write(value);
        };
        self.channel
            .ready
            .store(true, std::sync::atomic::Ordering::Release);
        self.receving_thread.unpark();
    }
}

unsafe impl<'a, T> Send for Sender<'a, T> {}

impl<'a, T> Recever<'a, T> {
    fn receive(&self) -> T {
        while !self
            .channel
            .ready
            .swap(false, std::sync::atomic::Ordering::Acquire)
        {
            thread::park();
        }
        unsafe { (*self.channel.message.get()).assume_init_read() }
    }
}

pub fn run() {
    let mut channel = Channel::new();
    thread::scope(|s| {
        let (sender, receiver) = channel.split();
        s.spawn(move || {
            sender.send("hello world!");
        });
        assert_eq!(receiver.receive(), "hello world!");
    });
}
