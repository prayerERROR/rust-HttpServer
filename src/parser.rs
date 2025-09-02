// parse http request
use std::collections::{HashSet, HashMap};

pub enum Command {
    Get,
    GetEcho(String),
    GetFile(String),
    GetUserAgent(String),
    PostFile(String, String),
    Error,
}

pub struct ParseResult {
    pub command: Command,
    pub is_close: bool,
    pub encoding: String,
}

impl ParseResult {
    fn new() -> Self {
        Self {
            command: Command::Error,
            is_close: false,
            encoding: "None".to_string(),
        }
    }
}

// main parse function
pub fn parse(raw_request: &str) -> ParseResult {
    let mut ret = ParseResult::new();

    // Step1 Split raw request to head and body
    let request: Vec<&str> = raw_request.split("\r\n\r\n").collect();
    if request.len() < 2 {
        return ret;
    }
    let request_head = request[0].trim();
    let request_body = request[1].trim();

    // Step2 Process request head
    // Split first line and others
    let lines: Vec<&str> = request_head.split("\r\n").collect();
    if lines.len() == 0 {
        return ret;
    }

    // Parse first line
    let first_line: Vec<&str> = lines[0].split_whitespace().collect();
    if first_line.len() != 3 {
        return ret;
    }
    let method = first_line[0];
    let path = first_line[1];
    let _version = first_line[2];

    // Parse other lines in request header
    let mut header_items: HashMap<String, String> = HashMap::new();
    for i in 1..lines.len() {
        let kv: Vec<&str> = lines[i].split(": ").collect();
        if kv.len() != 2 {
            return ret;
        }
        header_items.insert(kv[0].to_string(), kv[1].to_string());
    }

    // Check if close
    match header_items.get("Connection") {
        Some(val) if val == "close" => {
            ret.is_close = true;
        },
        _ => {},
    }

    // Check if accept encoding
    match header_items.get("Accept-Encoding") {
        Some(val) => {
            let methods: HashSet<&str> = val.split(", ").collect();
            if methods.contains("gzip") {
                ret.encoding = "gzip".to_string();
            }
        },
        _ => {},
    }

    // Return parse result
    ret.command = match method {
        "GET" => match path {
            path if path.starts_with("/echo/") => Command::GetEcho(path[6..].to_string()),
            path if path.starts_with("/files/") => Command::GetFile(path[7..].to_string()),
            "/user-agent" => match header_items.get("User-Agent") {
                Some(val) => Command::GetUserAgent(val.clone()),
                None => Command::Error,
            },
            "/" => Command::Get,
            _ => Command::Error,
        },
        "POST" => match path {
            path if path.starts_with("/files/") => {
                let file_name = path[7..].to_string();
                let contents = request_body.to_string();
                Command::PostFile(file_name, contents)
            },
            _ => Command::Error,
        },
        _ => Command::Error,
    };

    ret
}
