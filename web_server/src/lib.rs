use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool {
  workers : Vec<Worker>,
  sender: mpsc::Sender<Message>,
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
      let job = Box::new(f);
      self.sender.send(Message::NewJob(job)).unwrap();
    }

}

impl Drop for ThreadPool {
  fn drop(&mut self) {
    println!("Sending terminate message to all workers.");

    for _ in &self.workers { // send terminate N times, so N threads will receive one each
      self.sender.send(Message::Terminate).unwrap();
    }
    println!("Shutting down all workers.");
    
    // need two loops because when we send terminate we don't know which thread will get it
    // it may not be the same thread that we call thread.join() on, and join() is blocking,
    // so we may be blocking on a thread that didn't receive a terminate yet
    for worker in &mut self.workers {
      println!("Shutting down worker {}", worker.id);
      if let Some(thread) = worker.thread.take() {
        thread.join().unwrap(); // we know all threads have received a terminate from previous loop
      }   
    }
  
  
  }
}

// Worker is our light wrapper around a thread
struct Worker {
  id: usize,
  thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
  pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
    let thread = thread::spawn(move || loop { // loop forever to listen for incomming tasks
      let message = receiver.lock().unwrap().recv().unwrap(); // recv() blocks if no work present

      // by having recv() block this thread holds its place as next in line.
      // not problem that channel locked. it's only locked while no tasks are being sent across it
      match message {
        Message::NewJob(job) => {
          println!("Worker {} got a job; executing.", id);
          job(); // run the code passed to this thread
        }
        Message::Terminate => {
          println!("Worker {} was told to terminate.", id);
          break; // break infinite loop
        }
      }
    });
    
    return Worker { id:id, thread:Some(thread) };
  }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message { // to indicate if thread should take a new job or exit their infinite loop
  NewJob(Job),
  Terminate,
}