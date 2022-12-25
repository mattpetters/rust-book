/*
Mutexes have a reputation for being difficult to use because you have to remember two rules:

You must attempt to acquire the lock before using the data.
When you’re done with the data that the mutex guards, you must unlock
the data so other threads can acquire the lock.

Management of mutexes can be incredibly tricky to get right, which is why so many people are enthusiastic about channels.
However, thanks to Rust’s type system and ownership rules, you can’t get locking and unlocking wrong.

*/

/*
We did it! We counted from 0 to 10, which may not seem very impressive, but it did teach us
a lot about Mutex<T> and thread safety. You could also use this program’s structure to do
more complicated operations than just incrementing a counter. Using this strategy,
you can divide a calculation into independent parts, split those parts across threads,
and then use a Mutex<T> to have each thread update the final result with its part.

Similarly, Mutex<T> comes with the risk of creating deadlocks.
These occur when an operation needs to lock two resources and two threads
have each acquired one of the locks, causing them to wait for each other forever.
If you’re interested in deadlocks, try creating a Rust program that has a deadlock;
then research deadlock mitigation strategies for mutexes in any language and have a
go at implementing them in Rust. The standard library API documentation for
Mutex<T> and MutexGuard offers useful information.

*/

use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    // let m = Mutex::new(5);

    // {
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    // }

    // println!("m = {:?}", m);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
