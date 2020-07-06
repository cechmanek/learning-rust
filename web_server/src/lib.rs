use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool {
  workers : Vec<Worker>,
  sender: mpsc::Sender<Job>,
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
    assert!(size > 0); // doesn't make sense to have 0 threads
    
    let (sender, receiver) = mpsc::channel();

    let receiver = Arc::new(Mutex::new(receiver));

    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }
    return ThreadPool{ workers, sender };
  }

  pub fn execute<F>(&self, f: F) // define to take a closure as parameter
    where
      F: FnOnce() + Send + 'static, // FnOnce is one of three possible traits to use
    {
    }

}

// Worker is our light wrapper around a thread
struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>,
}

impl Worker {
  pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    let thread = thread::spawn(|| {
      receiver;
    }); // spawn with empty code block. needed to keep alive
    
    return Worker { id, thread };
  }
}

struct Job;