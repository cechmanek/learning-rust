/* simple examples using threads
*/

// libraries for threads
use std::thread;
use std::time::Duration;

// libraries for messge passing between threads
use std::sync::mpsc;

fn main() {
  thread::spawn (|| {
    for i in 1..10 { // this thread will not finish as once the main thread ends this all stops
      println!("side thread is on number {}", i);
      thread::sleep(Duration::from_millis(1));
    }
  });

  for i in 1..5 {
    println!("main thread is on number {}", i);
    thread::sleep(Duration::from_millis(1));
  }

  let second_thread = thread::spawn ( || {
    for i in 1..10 {
      println!("second side thread is on number {}", i);
      thread::sleep(Duration::from_millis(1));
    }
  });

  second_thread.join(); // block the main() function and wait until this thread is finished
  // waiting for second_thread to finish also gives more time to the first thread,
  // but still doesn't guarantee first thread will finish

  // if we want to pass data into a thread we must formally provide ownership or reference
  let my_vector = vec![1,2,3];
  let third_thread = thread::spawn( move || { // move grabs all needed variables from parent scope
    println!("the data in my_vector is {:?}", my_vector);
  });

  // as with all Rust move rules we can't use my_vector after it's owned by another function/thread
  //println!("the data in my_vector is {:?}", my_vector); // can't compile. use-after-move error 

  let result = third_thread.join().unwrap(); // join() returns an option, so unwrap it
  // since our threads don't formally return anything this option.unwrap() will be empty

  //////////////////////////////////////////
  // message passing with channels examples

  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let val = String::from("hi");
    tx.send(val).unwrap(); // send() returns a result, which is either OK or Err
  });

  let received = rx.recv().unwrap(); // similar to thread::join(). this blocks & waits for tx.send()
  println!("we got the transmission: {}", received);

  // the above example doesn't very clearly show the use of threads
  // really all you see is a single println! statement
  // so lets send multiple messages over the tx rx channel

  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let vals = vec![String::from("hi"),
                    String::from("from"),
                    String::from("the"),
                    String::from("thread")];
    for val in vals {
      tx.send(val).unwrap();
      thread::sleep(Duration::from_millis(1));
    } 
  });

  // now go through all the recieved values
  for received in rx { // if we don't access rx then main process may end before tx sends everything
    println!("got message: {}", received);
  }

  // to use the 'multiple producer' aspect of mpsc we need to clone the transmitter
  let (tx, rx) = mpsc::channel();
  let tx1 = mpsc::Sender::clone(&tx);

  // now we can spawn two threads, give each one a tx, and collect all outputs in one rx
  thread::spawn(move || {
    let vals = vec![String::from("hi"),
                    String::from("from"),
                    String::from("the"),
                    String::from("first"),
                    String::from("thread")];
    for val in vals {
      tx.send(val).unwrap();
      thread::sleep(Duration::from_millis(1));
    }
  });

  thread::spawn(move || {
    let vals = vec![String::from("\t\tHI"),
                    String::from("\t\tFROM"),
                    String::from("\t\tTHE"),
                    String::from("\t\tSECOND"),
                    String::from("\t\tTHREAD")];
    for val in vals {
      tx1.send(val).unwrap();
      thread::sleep(Duration::from_millis(1));
    }
  });

  for received in rx { // gets all transmissions from tx and tx1
    println!("got message: {}", received);
  }

}
