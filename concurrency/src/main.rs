use std::thread;

fn main() {

    //create a vector that tracks all the handles to threads that we create so we can close them later
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn( move || {
                println!("HELLO!");
            }
        );
        handles.push(handle)
    }

    for handle in handles {
        handle.join().unwrap();
    }
    return;


}