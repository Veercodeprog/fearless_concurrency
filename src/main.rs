// 1. Threading Model
// Rust (1:1 Threading Model):
//
// Rust uses a 1:1 threading model where each thread in Rust corresponds directly to a thread managed by the operating system. Each thread can execute independently and in parallel on different CPU cores. This model provides true parallelism.
// Example: If you spawn 10 threads in Rust, the OS manages 10 separate threads.
// JavaScript (Event Loop with Single Thread):
//
// JavaScript, especially in Node.js, uses a single-threaded, event-driven model with a non-blocking I/O. Instead of creating multiple OS threads, JavaScript uses an event loop to handle asynchronous operations like I/O.
// When an asynchronous operation (e.g., file reading) is initiated, it doesn't block the main thread. Instead, the operation is offloaded, and the event loop continues processing other tasks. Once the operation is complete, a callback is pushed onto the event queue to be processed when the main thread is available.
// Example: JavaScript handles concurrency by offloading tasks to the event loop rather than creating multiple OS threads.
//
use std::thread;
use std::time::Duration;
fn main() {
    //the application will create a new thread and run the code inside the closure in that new
    //thread.the application terminates before the new thread has a chance to print all its values.
    //to wait on it we can join the handle<()> returned by thread::spawn.
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!(" number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1)); // sleep for 1 millisecond
        }
    });
    //blocking thread
    handle.join().unwrap(); // why unwrap() is used here? as the associated thread may panic
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        // it will sleep for 1 millisecond on main thread
        thread::sleep(Duration::from_millis(1)); // sleep for 1 millisecond
    }
    // for j in 1..10 {
    //     thread::spawn(move || {
    //         println!("hi number {} from the spawned thread!", &j);
    //         thread::sleep(Duration::from_millis(1)); // sleep for 1 millisecond
    //     });
    // }
    // println!("Hello, world!");
}
