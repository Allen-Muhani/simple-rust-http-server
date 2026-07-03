#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Put,
    Patch,
    Other(String),
}

impl Method {
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
