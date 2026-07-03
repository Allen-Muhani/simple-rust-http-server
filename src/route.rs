use std::collections::HashMap;
use std::io::{Error};

use crate::http_request::HttpRequest;
use crate::http_response::HttpResponse;
use crate::method::Method;

/// A handler invoked when an incoming request matches a [`Route`].
///
/// Receives the parsed request and a mutable response to populate. Must be
/// `Send + Sync` since routes may be shared across connection-handling threads.
pub type RouteHandler = Box<dyn Fn(&HttpRequest, &mut HttpResponse) + Send + Sync>;


type Result<T> = std::result::Result<T, RouteError>;


#[derive(Debug, Clone)]
struct RouteError;

/// Maps an HTTP method and path pattern to the handler that serves it.
pub struct Route {
    /// The HTTP method this route matches (e.g. `"GET"`).
    pub method: String,
    /// The path pattern this route matches, e.g. `"/users"` or, with a path
    /// parameter, `"/location/:id"`.
    pub pattern: String,
    /// The handler invoked when this route matches an incoming request.
    pub handler: RouteHandler,
}

impl Route {
    /// Checks whether `method` and `path` match this route.
    ///
    /// `path` is matched against `pattern` segment by segment: a pattern
    /// segment starting with `:` (e.g. `:id`) captures whatever segment is
    /// in that position of `path`. Returns the captured path parameters on
    /// a match, or `None` if the method or segment count/literals differ.
    pub fn matches(&self, method: &Method, path: &str) -> Result<HashMap<String, String>> {
        if Method::parse(&self.method) != *method {
            return Err("Methods did not match");
        }

        let x = 

        // Filtering out empty segments means "/user", "/user/", and
        // "/user//" (or a doubled slash anywhere) all match the same way,
        // instead of a stray "/" changing the segment count.
        let mut pattern_segments = self.pattern.split('/').filter(|s| !s.is_empty());
        let mut path_segments = path.split('/').filter(|s| !s.is_empty());
        let mut params = HashMap::new();

        while let (Some(pattern), Some(path)) = (pattern_segments.next(), path_segments.next()) {
            if pattern == path {
                continue;
            }
            match pattern.strip_prefix(':') {
                Some(name) => {
                    params.insert(name.to_string(), path.to_string());
                }
                None => return Err("Path param does not exist, or path did not match pattern!"),
            }
        }

        if pattern_segments.next().is_some() || path_segments.next().is_some() {
            return Err("Path not equal to pattern!!");
        }

        return Ok(params);
    }
}
