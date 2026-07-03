use std::collections::HashMap;

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
    /// Path or query parameters extracted for this request, keyed by name.
    pub params: HashMap<String, String>,
    /// The request body, if any. Empty when no `Content-Length` was sent.
    pub body: String,
}

impl HttpRequest {
    /// Deserializes the request body as JSON into `T`.
    ///
    /// Returns `None` if the body is empty, not valid JSON, or doesn't
    /// match the shape of `T`.
    pub fn json<T: DeserializeOwned>(&self) -> Option<T> {
        serde_json::from_str(&self.body).ok()
    }
}
