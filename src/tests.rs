use std::collections::HashMap;

use super::*;
use crate::method::Method;

/// Builds a bare `HttpRequest` for `method`/`path` with no headers,
/// params, or body — tests fill in whatever else they need directly,
/// since every field on `HttpRequest` is `pub`.
fn request(method: Method, path: &str) -> HttpRequest {
    HttpRequest {
        method,
        path: path.to_string(),
        headers: Vec::new(),
        query_params: HashMap::new(),
        path_params: HashMap::new(),
        body: String::new(),
    }
}

/// Dispatches `req` the same way `handle_connection` does: starting from a
/// `200` response that the matched handler (or a `404`/error path) may
/// override.
fn run(routes: &[Route], req: HttpRequest) -> HttpResponse {
    let mut response = HttpResponse::default();
    response.status(200);
    dispatch(routes, req, &mut response);
    response
}

#[test]
fn health_check_returns_ok() {
    let routes = build_routes();
    let response = run(&routes, request(Method::Get, "/"));

    assert_eq!(response.status, 200);
    assert_eq!(response.body, "Service is live");
}

#[test]
fn get_user_reads_query_param() {
    let routes = build_routes();
    let mut req = request(Method::Get, "/user");
    req.query_params.insert("id".to_string(), "2".to_string());

    let response = run(&routes, req);

    assert_eq!(response.status, 200);
    assert_eq!(response.body, "user id: 2");
}

#[test]
fn get_user_without_id_is_bad_request() {
    let routes = build_routes();
    let response = run(&routes, request(Method::Get, "/user"));

    assert_eq!(response.status, 400);
}

#[test]
fn get_location_reads_path_param() {
    let routes = build_routes();
    let response = run(&routes, request(Method::Get, "/location/5"));

    assert_eq!(response.status, 200);
    assert_eq!(response.body, "location id: 5");
}

#[test]
fn create_user_reads_json_body() {
    let routes = build_routes();
    let mut req = request(Method::Post, "/users");
    req.body = r#"{"name":"Alice"}"#.to_string();

    let response = run(&routes, req);

    assert_eq!(response.status, 201);
    assert_eq!(response.body, "created user: Alice");
}

#[test]
fn create_user_rejects_invalid_json() {
    let routes = build_routes();
    let mut req = request(Method::Post, "/users");
    req.body = "not json".to_string();

    let response = run(&routes, req);

    assert_eq!(response.status, 400);
}

#[test]
fn unmatched_route_is_not_found() {
    let routes = build_routes();
    let response = run(&routes, request(Method::Get, "/nope"));

    assert_eq!(response.status, 404);
}

#[test]
fn trailing_and_doubled_slashes_still_match() {
    let routes = build_routes();

    let mut with_trailing_slash = request(Method::Get, "/user/");
    with_trailing_slash
        .query_params
        .insert("id".to_string(), "9".to_string());
    assert_eq!(run(&routes, with_trailing_slash).status, 200);

    let response = run(&routes, request(Method::Get, "/location//5"));
    assert_eq!(response.body, "location id: 5");
}
