use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};

fn main() {
    let v = Arc::new(Mutex::new(Vec::with_capacity(10)));
    let (sender, receiver): (Sender<_>, Receiver<_>) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));
    let mut handles = vec![];

    for id in 0..2 {
        let rx = receiver.clone();
        let v = v.clone();
        let handle = thread::spawn(move || loop {
            // let guard = rx.lock().unwrap().recv();
            match rx.lock().unwrap().recv() {
                Ok(msg) => {
                    println!("Thread {} got message {}", id, msg);
                    v.lock().unwrap().push(id);
                    // thread::sleep(Duration::from_millis(500));
                }

                Err(e) => {
                    println!("Thread {} got an error: {}", id, e);
                    break;
                }
            }
        });
        handles.push(handle);
    }

    for i in 0..100 {
        sender.send(format!("msg#{}#", i)).unwrap();
    }

    drop(sender);

    for handle in handles {
        handle.join().unwrap();
    }
    println!("{:?}", v);
}

//Situation 1:
// thread 1: lock() -> match (unlock())-> lock() -> ...
// automatically unlock() happens to when lock() out of the scope
// then, the next lock operation is executed by the same thread
// as a result, other threads have no time to acquire the lock of the channel

//Situation 2:
// thread 1: lock() -> match (unlock())-> doSomething()
// thread 2: lock() -> match (unlock())-> doSomething()
// ....
// doSomething() gives other threads to acquire the lock of the channel

//Situation 3: (similar to Situation 2)
// thread 1: lock() -> match => {unlock(); doSomething()} -> doOtherThing()
// thread 2: lock() -> match => {unlock(); doSomething()} -> doOtherThing()

// Conclusion:
// switching tasks between threads must guarantee
// the duration between the adjacent `lock()` operations
