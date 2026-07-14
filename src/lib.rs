use std::{
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver, Sender},
    },
    thread::{self, JoinHandle},
};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().expect("Failed to lock receiver").recv().expect("Failed to receive job");
                println!("Worker {} got a job; executing.", id);
                job();
            }
        });
        Worker { id, thread }
    }
}

pub struct Threadpool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}


impl Threadpool {
    /// Create a new Threadpool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> Threadpool {
        assert!(size > 0);

        let (sender, reciever) = mpsc::channel::<Job>(); // why :: after channel? because channel is a function in mpsc module, and we are calling it to create a new channel for sending and receiving jobs. The :: syntax is used to access the function from the module.
        let receiver = Arc::new(Mutex::new(reciever));

        let mut workers = Vec::with_capacity(size);

        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }

        Threadpool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).expect("Failed to send job to worker");
    }
}
