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
/// Returns the matched route along with its captured path parameters.
pub fn find_route<'a>(
    routes: &'a [Route],
    method: &Method,
    path: &str,
) -> Option<(&'a Route, HashMap<String, String>)> {
    routes
        .iter()
        .find_map(|route| route.matches(method, path).map(|params| (route, params)))
}
