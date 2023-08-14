// threads1.rs
//
// This program spawns multiple threads that each run for at least 250ms, and
// each thread returns how much time they took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.
//
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a
// hint.
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];
    let results_vec = vec![];
    let arc = Arc::new(Mutex::new(results_vec));
    for i in 0..10 {
        let arc = Arc::clone(&arc);
        handles.push(thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
            let time_since = start.elapsed().as_millis();
            let mut vector = arc.lock().unwrap();
            vector.push(time_since);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
    let mut results = &*(arc.lock().unwrap());
    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}
