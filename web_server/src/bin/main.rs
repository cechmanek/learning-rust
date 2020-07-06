/*
a single threaded web server following the guide in chapter 20 of The Rust Book
*/

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

use web_server::ThreadPool; // our custom struct

fn main() {
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // listen for requests at address
 
  let pool = ThreadPool::new(4); // our custom ThreadPool struct. max of 4 threads

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    //handle_connection(stream); // for single threaded version

    pool.execute(|| {
        handle_connection(stream);
    });
  }
}

fn handle_connection(mut stream: TcpStream) {
  let mut buffer = [0; 512]; // allocate a buffer for writing our read request into

  stream.read(&mut buffer).unwrap();

  println!("Request: {}", String::from_utf8_lossy(&buffer[..])); // 'lossy' replaces unknown chars

  let get = b"GET / HTTP/1.1\r\n"; //'b' means interpret as bytes. GET request for '/' page

  let (status_line, file_name) = if buffer.starts_with(get) {
      ("HTTP/1.1 200 OK\r\n\r\n", "hello.html") // must be implicit return
    }
    else {
      ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(file_name).unwrap();
    let response = format!("{}{}",status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap(); 
}

