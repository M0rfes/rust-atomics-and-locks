use std::thread;
pub fn run() {
    let nums = vec![1, 2, 3, 4, 5];
    thread::scope(|s| {
        s.spawn(|| {
            println!("{}", nums.len());
        });
        s.spawn(|| {
            let sum = nums.iter().sum::<usize>();
            println!("sum : {sum}")
        });
    });
}
