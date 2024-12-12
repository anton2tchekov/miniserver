use std::fmt;
use std::thread;

pub struct ThreadPool {
    threads: Vec<Worker>,
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

#[derive(Debug)]
pub struct PoolCreationError;

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "Error related to ThreadPool")
    }
}

impl Worker {
    fn new(id: usize) -> Self {
        let thread = thread::spawn(|| {});
        Worker { id, thread }
    }
}

impl ThreadPool {
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size > 0 {
            let mut threads = Vec::with_capacity(size);

            for i in 0..size {
                threads.push(Worker::new(i));
            }

            Ok(Self { threads })
        } else {
            Err(PoolCreationError)
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
