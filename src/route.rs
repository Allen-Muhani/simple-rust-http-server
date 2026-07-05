use std::collections::HashMap;

use crate::http_request::HttpRequest;
use crate::http_response::HttpResponse;
use crate::method::Method;


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

/// A handler invoked when an incoming request matches a [`Route`].
///
/// # Arguments
/// - `&HttpRequest` — the matched, fully parsed request (including any path
///   parameters captured by the route pattern).
/// - `&mut HttpResponse` — the response to populate; starts at whatever
///   status the caller set (`200` by default) and is expected to have its
///   body and, if needed, status overwritten.
///
/// Returns nothing; the handler communicates its result by mutating the
/// response. Must be `Send + Sync` since routes may be shared across
/// connection-handling threads.
pub type RouteHandler = Box<dyn Fn(&HttpRequest, &mut HttpResponse) + Send + Sync>;

impl Route {
    /// Checks whether `method` matches this route's method.
    ///
    /// # Arguments
    /// - `method` — the incoming request's parsed method.
    ///
    /// Returns `true` if it equals this route's `method` field (parsed the
    /// same way), `false` otherwise.
    pub fn matches_method(&self, method: &Method) -> bool {
        Method::parse(&self.method) == *method
    }

    /// Checks whether `path` matches this route's pattern, independent of
    /// method.
    ///
    /// `path` is matched against `pattern` segment by segment: a pattern
    /// segment starting with `:` (e.g. `:id`) captures whatever segment is
    /// in that position of `path`.
    ///
    /// # Arguments
    /// - `path` — the incoming request's path, e.g. `"/location/5"`.
    ///
    /// Returns `Some` map of captured path parameter names to their values
    /// on a match (empty if the pattern had no `:param` segments), or `None`
    /// if the segment count or a literal segment differs.
    pub fn matches_path(&self, path: &str) -> Option<HashMap<String, String>> {
        // Filtering out empty segments means "/user", "/user/", and
        // "/user//" (or a doubled slash anywhere) all match the same way,
        // instead of a stray "/" changing the segment count.
        let pattern_segments: Vec<&str> = self.pattern.split('/').filter(|s| !s.is_empty()).collect();
        let path_segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

        if pattern_segments.len() != path_segments.len() {
            return None;
        }

        let mut params = HashMap::new();
        for (pattern, path) in pattern_segments.iter().zip(path_segments.iter()) {
            if pattern == path {
                continue;
            }
            match pattern.strip_prefix(':') {
                Some(name) => {
                    params.insert(name.to_string(), path.to_string());
                }
                None => return None,
            }
        }

        Some(params)
    }
}
