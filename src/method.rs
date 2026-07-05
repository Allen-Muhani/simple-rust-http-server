/// An HTTP request method.
#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Put,
    Patch,
    /// Any method not otherwise recognized, holding its raw text (e.g.
    /// `"DELETE"`).
    Other(String),
}

impl Method {
    /// Parses a method name from a request line (e.g. `"GET"`).
    ///
    /// # Arguments
    /// - `s` — the method token as received on the wire, expected uppercase.
    ///
    /// Returns the matching [`Method`] variant, or `Method::Other(s)` if `s`
    /// isn't one of the recognized methods. Never fails.
    pub fn parse(s: &str) -> Method {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            "PUT" => Method::Put,
            "PATCH" => Method::Patch,
            other => Method::Other(other.to_string()),
        }
    }
}
