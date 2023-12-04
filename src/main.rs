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

fn sleep_sort(nums: Vec<u64>) -> Vec<u64> {
    let output = Arc::new(Mutex::new(Vec::<u64>::new()));
    let barrier = Arc::new(Barrier::new(nums.len()));

    let threads: Vec<_> = nums.into_iter().map(|n| {
        let res = output.clone();
        let c = Arc::clone(&barrier);
        spawn(move || {
            // Ensure all threads start at the same time
            c.wait();

            // Sleep and then add the data to the Vec
            sleep(Duration::from_millis(n * 5));
            res.lock().unwrap().push(n);
        })
    }).collect();

    // Wait for all the threads to finish
    for t in threads {
        t.join().unwrap();
    }

    // Return the data!
    let lock = Arc::try_unwrap(output).unwrap();
    lock.into_inner().unwrap()
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut nums = Vec::new();

    // Generate 100 random numbers between 0 and 100
    for _ in 0..1000 {
        nums.push(rng.gen_range(0..100))
    }

    // Create a set for each algo
    let nums_sleep = nums.clone();
    let mut nums_std = nums.clone();

    // Test the speed of the SleepSort
    let now = Instant::now();
    let sorted = sleep_sort(nums_sleep);
    println!("Sleep Sort:    {:3}.{:03}ms", now.elapsed().as_millis(), now.elapsed().as_micros() % 1000);
    assert!(is_sorted(&sorted));

    // Test out the regular TimSort sorting algoritm
    let now = Instant::now();
    nums_std.sort();
    println!("Standard Sort: {:3}.{:03}ms", now.elapsed().as_millis(), now.elapsed().as_micros() % 1000);
    assert!(is_sorted(&nums_std));
}
