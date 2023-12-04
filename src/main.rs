use std::thread::{sleep, spawn};
use std::time::{Duration, Instant};
use rand::Rng;
use std::sync::{Arc, Mutex, Barrier};

fn is_sorted<T>(data: &[T]) -> bool
where
    T: Ord,
{
    data.windows(2).all(|w| w[0] <= w[1])
}

fn sleepsort(nums: Vec<u64>) -> Vec<u64> {
    let output = Arc::new(Mutex::new(Vec::<u64>::new()));

    let barrier = Arc::new(Barrier::new(nums.len()));
    let threads: Vec<_> = nums.into_iter().map(|n| {
        let res = output.clone();
        let c = Arc::clone(&barrier);
        spawn(move || {
            c.wait(); // Ensure all threads start at the same time
            sleep(Duration::from_millis(n * 5));
            res.lock().unwrap().push(n);
        })
    }).collect();

    for t in threads {
        t.join().unwrap(); // Wait for all the threads to finish
    }

    let lock = Arc::try_unwrap(output).unwrap();
    lock.into_inner().unwrap()
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut nums = Vec::new();
    for _ in 0..100 {
        nums.push(rng.gen_range(0..10))
    }

    let nums_sleep = nums.clone();
    let mut nums_std = nums.clone();

    for _ in 0..100 {
        let sort_set = nums_sleep.clone();
        let now = Instant::now();
        let sorted = sleepsort(sort_set);
        println!("Sleep Sort: {:3}.{:03}ms", now.elapsed().as_millis(), now.elapsed().as_micros() % 1000);
        assert!(is_sorted(&sorted));
    }

    let now = Instant::now();
    nums_std.sort();
    println!("Standard Sort: {:3}.{:03}ms", now.elapsed().as_millis(), now.elapsed().as_micros() % 1000);
    assert!(is_sorted(&nums_std));
}
