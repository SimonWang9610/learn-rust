/// -> ThreadPool:new(n)
///     -> Worker::new(id, receiver)
///         -> thread (keep looping)
///     -> sender
///
/// -> ThreadPool.execute(closure)
///     -> sender(closure)
///     -> received by Worker.thread
use std::fmt;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

// executable closure for handling each connection
type Job = Box<dyn FnOnce() + Send + 'static>;

// control the loop of Worker.thread
enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        // create the channel between the pool and threads
        let (sender, receiver) = mpsc::channel();
        // single(sender) -> multiple(receivers)
        // must wrap the receiver in Arc<Mutex>
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    /* pub fn new_version(size: usize) -> Result<ThreadPool, PoolCreationError<'static>> {
        if size <= 0 {
            Err(PoolCreationError("Fail to create pool!"))
        } else {
            Ok(ThreadPool)
        }
    } */

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending Terminate message to all workers...");

        // guarantee all threads receive Message::Terminate
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers...");

        for worker in &mut self.workers {
            println!("Shutting down worker #{}#...", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

pub struct Worker {
    id: usize,
    // why Option?
    // with no Option, we can not thread::JoinHandle::join() when dropping ThreadPool
    // because we cannot take the thread ownership of Worker
    // when ThreadPool.workers as a mutable reference
    // but `Option<T>.take()` can help take out the value of Some(value) with ownership
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    // threads keep alive and try to get a new job in a loop
    // a new job is passed by sender -> receiver
    // once the job is captured by one thread, the data is locked and become exclusive
    // then, the current thread is blocked and can not capture the message
    // until it completes the task and becomes available
    // meanwhile, other threads keep looping...
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing...", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} will be terminated!", id);
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
