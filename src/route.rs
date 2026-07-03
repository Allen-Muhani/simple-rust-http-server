use std::collections::HashMap;

use crate::http_request::HttpRequest;
use crate::http_response::HttpResponse;
use crate::method::Method;

/// A handler invoked when an incoming request matches a [`Route`].
///
/// Receives the parsed request and a mutable response to populate. Must be
/// `Send + Sync` since routes may be shared across connection-handling threads.
pub type RouteHandler = Box<dyn Fn(&HttpRequest, &mut HttpResponse) + Send + Sync>;

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
    pub fn matches(&self, method: &Method, path: &str) -> Option<HashMap<String, String>> {
        if Method::parse(&self.method) != *method {
            return None;
        }

        let mut pattern_segments = self.pattern.split('/');
        let mut path_segments = path.split('/');
        let mut params = HashMap::new();

        loop {
            match (pattern_segments.next(), path_segments.next()) {
                (Some(pattern_segment), Some(path_segment)) => {
                    match pattern_segment.strip_prefix(':') {
                        Some(name) => {
                            params.insert(name.to_string(), path_segment.to_string());
                        }
                        None if pattern_segment == path_segment => {}
                        None => return None,
                    }
                }
                (None, None) => return Some(params),
                _ => return None,
            }
        }
    }
}
