// Response
use std::{
    fs,
    path::Path,
};

pub fn response_get(is_close: bool) -> String {
    match is_close {
        true => "HTTP/1.1 200 OK\r\nConnection: close\r\n\r\n".to_string(),
        false => "HTTP/1.1 200 OK\r\n\r\n".to_string(),
    }
}

pub fn response_error(is_close: bool) -> String {
    match is_close {
        true => "HTTP/1.1 404 Not Found\r\nConnection: close\r\n\r\n".to_string(),
        false => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
    }
}

pub fn response_get_echo(msg: &str, is_close: bool) -> String {
    match is_close {
        true => format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\
            Content-Length: {}\r\nConnection: close\r\n\r\n{}", msg.len(), msg
        ), 
        false => format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\
            Content-Length: {}\r\n\r\n{}", msg.len(), msg
        ),
    }
}

pub fn response_get_user_agent(msg: &str, is_close: bool) -> String {
    match is_close {
        true => format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\
            Content-Length: {}\r\nConnection: close\r\n\r\n{}", msg.len(), msg
        ),
        false => format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\
            Content-Length: {}\r\n\r\n{}", msg.len(), msg
        ),
    }
   
}

pub fn response_get_file(file: &str, is_close: bool) -> String {
    let file_path =  Path::new(file);
    match file_path.is_file() {
        true => {
            let content = fs::read_to_string(file_path).unwrap();
            match is_close {
                true => format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\n\
                    Content-Length: {}\r\nConnection: close\r\n\r\n{}", content.len(), content
                ),
                false => format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\n\
                    Content-Length: {}\r\n\r\n{}", content.len(), content
                ),
            }
        },
        false => {
            match is_close {
                true => "HTTP/1.1 404 Not Found\r\nConnection: close\r\n\r\n".to_string(),
                false => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
            }
        },
    }
}

pub fn response_post_file(file: &str, contents: &str, is_close: bool) -> String {
    let mut ret = match fs::write(file, contents) {
        Ok(_) => "HTTP/1.1 201 Created".to_string(),
        Err(_) => "HTTP/1.1 500 Internal Server Error".to_string(),
    };

    if is_close {
        ret.push_str("\r\nConnection: close\r\n\r\n");
    } else {
        ret.push_str("\r\n\r\n");
    }
    ret
}