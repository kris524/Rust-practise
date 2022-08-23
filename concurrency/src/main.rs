use std::thread;

//an object that is used to ensure Mutual Exclusion -> only one thread has access to a resource at any given time
use std::sync::{Mutex, Arc};

fn main() {
    //create the mutex with integer value 0 as the controlled resource
    let counter = Arc::new(Mutex::new(0));


    //create a vector that tracks all the handles to threads that we create so we can close them later
    let mut handles = vec![];

    //spawn 10 threads
    for _ in 0..10 {

        let counter = Arc::clone(&counter);
        //spawn a thread using the library
        let handle = thread::spawn( move || {

            //
            let mut num = counter.lock().unwrap();
            *num += 1
                
            }
        );
        //add the newly created handle into the list of handles
        handles.push(handle)
    }

    //call join on all the handles and wait for them to finish executing 
    for handle in handles {
        handle.join().unwrap();
    }
    println!("RESULT: {}", *counter.lock().unwrap());
    return;


}

//In multithreded rust programing, we cant use a mutex on its own, 
//We need to wrap the mutex using a reference structure to be passed by value into out movement closure 