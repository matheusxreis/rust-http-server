use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn build(size: usize) -> Result<ThreadPool, &'static str> {
        if size == 0 {
            return Err("size can't be equal 0");
        }

        // multiple producer, single consumer
        // Arc type allows multiple receiver
        // Mutex ensures that only one worker gets a job from the receiver at a time
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers: Vec<Worker> = Vec::with_capacity(size);
        for id in 0..size {
            if let Ok(w) = Worker::build(id, Arc::clone(&receiver)) {
                workers.push(w)
            }
        }
        Ok(ThreadPool { workers, sender })
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

#[derive(Debug)]

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn build(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Result<Worker, &'static str> {
        let builder = thread::Builder::new();
        let mut thread: thread::JoinHandle<()>;
        if let Ok(join_handler) = builder.spawn(move || loop {
            let job = receiver
                .lock()
                .expect("Lock error")
                .recv()
                .expect("Recv error");
            println!("Worker {id} got a job; executing.");

            job()
        }) {
            thread = join_handler;
            return Ok(Worker { id, thread });
        }
        Err("Error when create worker")
    }
}
