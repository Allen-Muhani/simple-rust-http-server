use std::collections::HashMap;
use std::io::{self, Write};

use serde::Serialize;

/// An HTTP response being built by a route handler, populated via
/// [`send`](HttpResponse::send), [`json`](HttpResponse::json), and/or
/// [`status`](HttpResponse::status) before being written back to the client.
#[derive(Default)]
pub struct HttpResponse {
    /// The HTTP status code to send (e.g. `200`, `404`). Defaults to `0`
    /// until set via [`HttpResponse::status`].
    pub status: u16,
    /// Response headers, keyed by header name.
    pub headers: HashMap<String, String>,
    /// The response body to send back to the client.
    pub body: String,
}

impl HttpResponse {
    /// Sets the response body, converting `body` into a `String`.
    ///
    /// # Arguments
    /// - `body` — the raw response body text.
    ///
    /// Returns nothing; sets `self.body`.
    pub fn send(&mut self, body: impl Into<String>) {
        self.body = body.into();
    }

    /// Serializes `value` as JSON into the body and sets the
    /// `Content-Type` header to `application/json`.
    ///
    /// # Arguments
    /// - `value` — the value to serialize as the response body.
    ///
    /// Returns nothing; sets `self.body` and `self.headers`. If
    /// serialization fails, the body is set to an empty string rather than
    /// panicking.
    pub fn json<T: Serialize>(&mut self, value: &T) {
        self.headers
            .insert("Content-Type".into(), "application/json".into());
        self.body = serde_json::to_string(value).unwrap_or_default();
    }

    /// Sets the HTTP status code.
    ///
    /// # Arguments
    /// - `code` — the status code to send, e.g. `200` or `404`.
    ///
    /// Returns nothing; sets `self.status`.
    pub fn status(&mut self, code: u16) {
        self.status = code;
    }

    /// Writes this response as raw HTTP bytes to `writer` (e.g. a `TcpStream`).
    ///
    /// # Arguments
    /// - `writer` — the destination to write the status line, headers, and
    ///   body to.
    ///
    /// Returns `Ok(())` on success, or `Err` if the underlying write fails.
    pub fn write_to(&self, mut writer: impl Write) -> io::Result<()> {
        let mut head = format!(
            "HTTP/1.1 {} {}\r\n",
            self.status,
            status_reason(self.status)
        );
        for (name, value) in &self.headers {
            head.push_str(&format!("{name}: {value}\r\n"));
        }
        head.push_str(&format!("Content-Length: {}\r\n\r\n", self.body.len()));

        writer.write_all(head.as_bytes())?;
        writer.write_all(self.body.as_bytes())
    }
}

/// Maps a status code to its standard reason phrase (e.g. `200` -> `"OK"`).
///
/// # Arguments
/// - `status` — the status code to look up.
///
/// Returns the reason phrase, or `"Unknown"` if `status` isn't one of the
/// codes this server sends.
fn status_reason(status: u16) -> &'static str {
    match status {
        200 => "OK",
        201 => "Created",
        400 => "Bad Request",
        404 => "Not Found",
        500 => "Internal Server Error",
        _ => "Unknown",
    }
}
