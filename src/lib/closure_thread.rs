use std::thread;

pub fn run() {
    let vec = vec![1, 2, 3];
    let avg = thread::spawn(move || {
        let len = vec.len();
        let sum = vec.iter().sum::<usize>();
        sum / len
    })
    .join()
    .unwrap();
    println!("the average is {avg}");
}
