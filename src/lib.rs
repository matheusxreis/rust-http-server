use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers: Vec<Worker> = Vec::with_capacity(size);
        for id in 0..size {
            if let Ok(w) = Worker::build(id) {
                workers.push(w)
            }
        }
        ThreadPool { workers }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

#[derive(Debug)]

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn build(id: usize) -> Result<Worker, &'static str> {
        let builder = thread::Builder::new();
        let mut thread: thread::JoinHandle<()>;
        if let Ok(join_handler) = builder.spawn(|| {}) {
            thread = join_handler;
            return Ok(Worker { id, thread });
        }
        Err("Error when create worker")
    }
}
