use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Result};
use std::net::TcpStream;

use serde::de::DeserializeOwned;

use crate::method::Method;

/// A parsed HTTP request received from a client connection.
#[derive(Debug)]
pub struct HttpRequest {
    /// The HTTP method from the request line (e.g. `GET`, `POST`).
    pub method: Method,
    /// The request target path from the request line (e.g. `/users`).
    pub path: String,
    /// Header fields as `(name, value)` pairs, in the order received.
    pub headers: Vec<(String, String)>,
    /// Query string parameters (e.g. `?id=5` -> `{"id": "5"}`), parsed
    /// directly from the request line.
    pub query_params: HashMap<String, String>,
    /// Path parameters captured from a route pattern (e.g. `/location/:id`
    /// matching `/location/5` -> `{"id": "5"}`). Empty until the router
    /// matches this request to a route and fills them in.
    pub path_params: HashMap<String, String>,
    /// The request body, if any. Empty when no `Content-Length` was sent.
    pub body: String,
}

impl HttpRequest {
    /// Parses an `HttpRequest` out of a readable stream (e.g. a `TcpStream`).
    ///
    /// Reads the request line (`METHOD /path?query HTTP/1.1`), then header
    /// lines up to the blank line that separates headers from the body, then
    /// reads exactly `Content-Length` bytes of body if that header was sent.
    /// Query parameters on the path (e.g. `?id=5`) are parsed into `params`.
    ///
    /// Returns `Ok(None)` if the connection had no request line to read
    /// (e.g. the client closed the connection immediately). Returns `Err`
    /// if reading from `stream` fails or a header line isn't valid UTF-8.
    pub fn from_stream(stream: &TcpStream) -> Result<Option<Self>> {
        let mut reader = BufReader::new(stream);
        let mut lines = reader.by_ref().lines();

        // The request line is the first line, e.g. "GET /users?id=5 HTTP/1.1".
        // Its absence (empty read or EOF) means there's no request to parse.

        let request_line = match lines.next() {
            Some(Ok(line)) => line,
            Some(Err(e)) => {
                eprintln!("Failed to read line from connection: {e}");
                return Err(e);
            }
            None => return Ok(None),
        };
        if request_line.is_empty() {
            return Ok(None);
        }

        let mut parts = request_line.split_whitespace();
        let method = Method::parse(parts.next().unwrap_or(""));
        let raw_path = parts.next().unwrap_or("");

        // Split the query string (if any) off the path so `path` stays a
        // plain route like "/users" and query params land in `query_params`.
        let (path, query_params) = match raw_path.split_once('?') {
            Some((path, query)) => (path.to_string(), parse_query_string(query)),
            None => (raw_path.to_string(), HashMap::new()),
        };

        let (headers, content_length) = parse_headers(&mut lines)?;
        let body = read_body(&mut reader, content_length)?;

        Ok(Some(Self {
            method,
            path,
            headers,
            query_params,
            path_params: HashMap::new(),
            body,
        }))
    }

    /// Deserializes the request body as JSON into `T`.
    ///
    /// Returns `None` if the body is empty, not valid JSON, or doesn't
    /// match the shape of `T`.
    pub fn json<T: DeserializeOwned>(&self) -> Option<T> {
        serde_json::from_str(&self.body).ok()
    }

    /// Records path parameters captured by a matched route (e.g.
    /// `/location/:id` matching `/location/5` captures `id -> "5"`).
    ///
    /// A no-op if `path_params` is empty, i.e. the matched route's pattern
    /// had no `:param` segments to capture.
    pub fn set_path_params(&mut self, path_params: HashMap<String, String>) {
        if !path_params.is_empty() {
            self.path_params = path_params;
        }
    }
}

/// Parses a URL query string (e.g. `id=5&sort=asc`) into name/value pairs.
///
/// Entries without an `=` are skipped rather than treated as an error.
fn parse_query_string(query: &str) -> HashMap<String, String> {
    query
        .split('&')
        .filter_map(|pair| pair.split_once('='))
        .map(|(name, value)| (name.to_string(), value.to_string()))
        .collect()
}

/// Reads header lines from `lines` until the blank line that ends them,
/// returning the collected headers and the `Content-Length` value seen
/// (`0` if the header wasn't present).
fn parse_headers(
    lines: &mut impl Iterator<Item = Result<String>>,
) -> Result<(Vec<(String, String)>, usize)> {
    let mut headers = Vec::new();
    let mut content_length: usize = 0;
    for line in lines {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                eprintln!("Failed to read header line from connection: {e}");
                return Err(e);
            }
        };
        if line.is_empty() {
            break;
        }
        if let Some((name, value)) = line.split_once(": ") {
            if name.eq_ignore_ascii_case("Content-Length") {
                content_length = value.trim().parse().unwrap_or(0);
            }
            headers.push((name.to_string(), value.to_string()));
        }
    }
    Ok((headers, content_length))
}

/// Reads exactly `content_length` bytes from `reader` as the request body.
///
/// Returns an empty string without reading anything if `content_length` is
/// `0`, so bodyless requests (e.g. `GET`) never block waiting for bytes
/// that will never arrive.
fn read_body(reader: &mut impl Read, content_length: usize) -> Result<String> {
    if content_length == 0 {
        return Ok(String::new());
    }

    let mut buf = vec![0u8; content_length];
    match reader.read_exact(&mut buf) {
        Ok(()) => Ok(String::from_utf8_lossy(&buf).into_owned()),
        Err(e) => {
            eprintln!("Failed to read request body from connection: {e}");
            Err(e)
        }
    }
}
