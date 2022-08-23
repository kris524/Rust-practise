use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("The number we are at the spawned thread {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // handle.join().unwrap();
    //if the main thread ends, the spawn thread stops
    for i in 1..5 {
        println!("The number we are at the main thread {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    //that is why we use a handle to block the thread currently running (main thread)
    //until the thread asociated with the handle (the spawn thread) terminates
    handle.join().unwrap();

    let v = vec![1, 2, 3];
                                                //let the closure take ownership of v,
                                                // that means the main thread can no longer use v
    let handle2 = thread::spawn(move || {
        println!("Heres a vector {:?}", v)
    });

    handle2.join().unwrap();
}
