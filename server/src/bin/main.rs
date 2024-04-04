// Our server will be bare-bones. We'll use the two main protocols involved in web servers are Hypertext Transfer Protocol (HTTP) and Transmission Control Protocol (TCP). 
// Both protocols are request-response protocols, meaning a client initiates requests and a server listens to the requests and provides a response to the client. The contents of those requests and responses are defined by the protocols.

// SINGLE THREADED HTTP SERVER
// ===========================
use std::{
    fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}
};

fn main() {
    // main_single_threaded();
    main_multi_threaded();
}

fn main_single_threaded() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection_5(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);
}

// BASIC WAY
fn handle_connection_2(mut stream: TcpStream){
    let mut buffer = [0; 1024]; // just large enough to read the request
    stream.read(&mut buffer).unwrap(); // read the request and put it in the buffer
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..])); // convert the buffer to a string and print it

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap(); // flush the stream to ensure all data is written to the connection

    // Now we will get a request from the browser and send a response back to the browser.
}

// RETURNING HTML IN RESPONSE
fn handle_connection_3(mut stream: TcpStream){
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let contents = fs::read_to_string("index.html").unwrap();   

    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", contents.len(), contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

// VALIDATING THE REQUEST
fn handle_connection_4(mut stream: TcpStream){
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";  // b" " is a byte string literal - it's a byte array
    
    let (status_line, filename) = if buffer.starts_with(get){
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

// We’re going to implement sending data in response to a client request. Responses must have the following format:
// HTTP-Version Status-Code Reason-Phrase CRLF (here CRLF means a carriage return followed by a line feed, which is how newlines are represented in HTTP)
// headers CRLF
// message-body

// Here is an example response that uses HTTP version 1.1, has a status code of 200, an OK reason phrase, no headers, and no body:
// HTTP/1.1 200 OK\r\n\r\n  (\r\n is the CRLF)

// ============================================================

// MULTITHREADED HTTP SERVER
// ==========================

use std::thread;
use std::time::Duration;

// sleep function is used to simulate a slow request - this will block the thread for 5 seconds
fn handle_connection_5(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";  // b" " is a byte string literal - it's a byte array
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    
    let (status_line, filename) = if buffer.starts_with(get){
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else if buffer.starts_with(sleep){
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

// thread pool implementation - it will create a pool of threads and distribute incoming requests among them
// we should not spawn a new thread for each request because it will consume a lot of resources

use server::ThreadPool;

fn main_multi_threaded(){
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2)  { // shut down after 2 requests
        // so after 2 requests, the program will exit and clean up the threads
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection_5(stream);
        });
    }
}