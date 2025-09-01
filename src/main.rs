#[allow(unused_imports)]
use std::{
    env,
    io::{Read, Write},
    net::{TcpStream, TcpListener},
};
use threadpool::ThreadPool;

mod parser;
mod response;

use parser::*;
use response::*;

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
                let raw_request = std::str::from_utf8(&buffer[..n]).unwrap();
                let parse_result = parse(raw_request);
                let is_close = parse_result.is_close;

                let reply = match parse_result.command {
                    Command::Get => response_get(is_close),
                    Command::GetUserAgent(msg) => response_get_user_agent(&msg, is_close),
                    Command::GetEcho(msg) => response_get_echo(&msg, is_close),
                    Command::GetFile(file_name) =>
                        response_get_file(&format!("{file_dir}{file_name}"), is_close),
                    Command::PostFile(file_name, contents) =>
                        response_post_file(&format!("{file_dir}{file_name}"), &contents, is_close),
                    Command::Error => response_error(is_close),
                };

                stream.write_all(reply.as_bytes()).unwrap();
                stream.flush().unwrap();

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

