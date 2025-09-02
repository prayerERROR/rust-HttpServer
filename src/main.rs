#[allow(unused_imports)]
use threadpool::ThreadPool;
use std::{
    env,
    io::{Read, Write},
    net::{TcpStream, TcpListener},
};

mod parser;
mod response;

use parser::parse;
use response::generate_response;

// main
fn main() {
    // Read exec arguments
    let args: Vec<String> = env::args().collect();
    let file_dir = match args.get(2) {
        Some(dir) => dir.clone(),
        None => String::new(),
    };

    // Initialize tcp listener and thread pool
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    let pool = ThreadPool::new(64);

    // Handle requests
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let file_dir_clone = file_dir.clone();
                pool.execute(move || {
                    handle_connection(stream, file_dir_clone);
                });
            },
            Err(e) => {
                println!("error: {}", e);
            },
        }
    }
}

// handle connections
fn handle_connection(mut stream: TcpStream, file_dir: String) {
    loop {
        let mut buffer = [0u8; 512];
        match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                // Parse request and generate response
                let raw_request = std::str::from_utf8(&buffer[..n]).unwrap();
                let parse_result = parse(raw_request);
                let is_close = parse_result.is_close;
                let response = generate_response(&file_dir, parse_result);

                // Return response
                stream.write_all(&response).unwrap();
                stream.flush().unwrap();

                // Check if close tcp connection
                if is_close {
                    break;
                }
            },
            Err(e) => {
                println!("error: {}", e);
            },
        }
    }
}

