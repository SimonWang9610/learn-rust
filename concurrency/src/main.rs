use std::rc::Rc;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
    /* let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("vector {:?}", v);
        let a = Test::a;
        return (v, a);
    });

    for i in 1..5 {
        println!("number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    match handle.join() {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{:?}", e),
    } */

    /* let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("hello"),
            String::from("Simon"),
            String::from("the"),
            String::from("spawned"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hello tx1"),
            String::from("Simon tx1"),
            String::from("the tx1"),
            String::from("spawned tx1"),
            String::from("thread tx1"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for received in rx {
        println!("Got {}", received);
    } */

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 1..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("Counter: {:?}", counter);
        });
        handles.push(handle);
    }
    println!("Counter: {:?}", counter);

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

// #[derive(Debug)]
// enum Test {
//     a,
// }
