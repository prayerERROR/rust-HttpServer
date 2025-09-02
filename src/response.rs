// Response
use flate2::{
    write::GzEncoder,
    Compression,
};
use std::{
    fs,
    path::Path,
    io::Write,
};

use crate::parser::{Command, ParseResult};


pub fn generate_response(file_dir: &str, parse_result: ParseResult) -> Vec<u8> {
    // Generate response header
    let (mut body, mut body_encoded) = (String::new(), Vec::<u8>::new()); // Store response body message
    
    let mut header = match parse_result.command {
        Command::Get => "HTTP/1.1 200 OK\r\n".to_string(),
        Command::GetEcho(msg) => {
            body.push_str(&msg);
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n".to_string()
            // Content-Length: {}\r\n", msg.as_bytes().len())
        },
        Command::GetFile(file_name) => {
            let file = format!("{file_dir}{file_name}");
            let file_path = Path::new(&file);
            match file_path.is_file() {
                true => {
                    let content = fs::read_to_string(file_path).unwrap();
                    body.push_str(&content);
                    "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\n".to_string()
                },
                false => "HTTP/1.1 404 Not Found\r\n".to_string(),
            }
        },
        Command::GetUserAgent(msg) => {
            body.push_str(&msg);
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n".to_string()
        },
        Command::PostFile(file_name, contents) => {
            let file = format!("{file_dir}{file_name}");
            let file_path = Path::new(&file);
            match fs::write(file_path, contents) {
                Ok(_) => "HTTP/1.1 201 Created\r\n".to_string(),
                Err(_) => "HTTP/1.1 500 Internal Server Error\r\n".to_string(),
            }
        },
        Command::Error => "HTTP/1.1 404 Not Found\r\n".to_string(),   
    };

    // Add other lines to response header
    // Add "content length" and "content encoding"
    if body.len() > 0 {
        // body(String) -> body_encoded (Vec<u8>)
        body_encoded = match parse_result.encoding {
            val if val == "gzip" => {
                header.push_str(&"Content-Encoding: gzip\r\n".to_string());
                let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
                encoder.write_all(body.as_bytes()).unwrap();
                encoder.finish().unwrap()
            },
            _ => body.into_bytes(),
        };
        let content_line = format!("Content-Length: {}\r\n", body_encoded.len());
        header.push_str(&content_line);
    } 

    // Add "conection"
    if parse_result.is_close {
        header.push_str("Connection: close\r\n");
    }
    
    // Add an empty line "\r\n" to split header and body
    header.push_str("\r\n");
    let mut header_bytes = header.into_bytes();

    // Add response body
    if body_encoded.len() > 0 {
        header_bytes.extend(body_encoded);
    }

    // Return response
    header_bytes
}

