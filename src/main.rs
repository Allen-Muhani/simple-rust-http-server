use std::{
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
};

mod http_request;
mod method;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => stream,
            Err(e) => {
                eprintln!("Failed to accept connection: {e}");
                continue;
            }
        };

        handle_connection(stream);
    }
}

/// Reads an incoming HTTP request from `stream` line by line and prints it.
///
/// Lines are read until the first empty line, which marks the end of the
/// HTTP request headers (the request line plus header fields).
fn handle_connection(stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);

    let mut http_request: Vec<String> = Vec::new();

    for line in buf_reader.lines() {
        match line {
            Ok(line) if line.is_empty() => break,
            Ok(line) => http_request.push(line),
            Err(e) => {
                eprint!("Failed to read line from connection: {e}");
                return;
            }
        }
    }

    println!("request: {http_request:#?}");
}
