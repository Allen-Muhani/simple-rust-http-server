use std::collections::HashMap;

use crate::handlers;
use crate::method::Method;
use crate::route::Route;

/// Builds the application's routing table.
///
/// - `GET /` — health check.
/// - `GET /user` — reads `id` from the query string (e.g. `/user?id=2`).
/// - `GET /location/:id` — reads `id` from the path.
/// - `POST /users` — reads a JSON body.
///
/// Takes no arguments. Returns the fixed list of [`Route`]s in match
/// priority order (earlier entries are tried first).
pub fn build_routes() -> Vec<Route> {
    vec![
        Route {
            method: "GET".to_string(),
            pattern: "/".to_string(),
            handler: handlers::health(),
        },
        Route {
            method: "GET".to_string(),
            pattern: "/user".to_string(),
            handler: handlers::get_user(),
        },
        Route {
            method: "GET".to_string(),
            pattern: "/location/:id".to_string(),
            handler: handlers::get_location(),
        },
        Route {
            method: "POST".to_string(),
            pattern: "/users".to_string(),
            handler: handlers::create_user(),
        },
    ]
}

/// Finds the first route in `routes` matching `method` and `path`.
///
/// # Arguments
/// - `routes` — the routing table to search, in priority order.
/// - `method` — the incoming request's parsed method.
/// - `path` — the incoming request's path (query string already stripped).
///
/// Returns `Ok` with the matched route and its captured path parameters
/// (empty if the pattern had none), or `Err` with a message if no route in
/// `routes` matches both `method` and `path`.
pub fn find_route<'a>(
    routes: &'a [Route],
    method: &Method,
    path: &str,
) -> Result<(&'a Route, HashMap<String, String>), String> {
    routes
        .iter()
        .filter(|route| route.matches_method(method))
        .find_map(|route| route.matches_path(path).map(|params| (route, params)))
        .ok_or_else(|| "Route not found".to_string())
}