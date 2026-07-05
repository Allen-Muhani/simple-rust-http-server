use serde::Deserialize;

use crate::http_request::HttpRequest;
use crate::http_response::HttpResponse;
use crate::route::RouteHandler;

/// Handles the default `/` route, confirming the server is running.
///
/// Takes no arguments. Returns a [`RouteHandler`] that ignores the request
/// and always sends a `200` with body `"Service is live"`.
pub fn health() -> RouteHandler {
    Box::new(|_req: &HttpRequest, res: &mut HttpResponse| {
        res.send("Service is live");
    })
}

/// Handles `GET /user?id=<id>`, reading `id` from the query string.
///
/// Takes no arguments. Returns a [`RouteHandler`] that, given a request:
/// - reads `id` from `req.query_params` and sends `200` with body
///   `"user id: {id}"` if present;
/// - otherwise sends `400` with an explanatory body.
pub fn get_user() -> RouteHandler {
    Box::new(
        |req: &HttpRequest, res: &mut HttpResponse| match req.query_params.get("id") {
            Some(id) => res.send(format!("user id: {id}")),
            None => {
                res.status(400);
                res.send("missing \"id\" query param");
            }
        },
    )
}

/// Handles `GET /location/:id`, reading `id` from the path.
///
/// Takes no arguments. Returns a [`RouteHandler`] that, given a request:
/// - reads `id` from `req.path_params` (captured by the router from the
///   `:id` pattern segment) and sends `200` with body `"location id: {id}"`
///   if present;
/// - otherwise sends `400` with an explanatory body.
pub fn get_location() -> RouteHandler {
    Box::new(
        |req: &HttpRequest, res: &mut HttpResponse| match req.path_params.get("id") {
            Some(id) => res.send(format!("location id: {id}")),
            None => {
                res.status(400);
                res.send("missing \"id\" path param");
            }
        },
    )
}

/// The JSON body expected by [`create_user`].
#[derive(Deserialize)]
pub struct CreateUserBody {
    pub name: String,
}

/// Handles `POST /users`, deserializing the request body as JSON.
///
/// Takes no arguments. Returns a [`RouteHandler`] that, given a request:
/// - deserializes `req.body` as [`CreateUserBody`] and, on success, sends
///   `201` with body `"created user: {name}"`;
/// - otherwise (empty, invalid JSON, or wrong shape) sends `400` with an
///   explanatory body.
pub fn create_user() -> RouteHandler {
    Box::new(
        |req: &HttpRequest, res: &mut HttpResponse| match req.json::<CreateUserBody>() {
            Some(body) => {
                res.status(201);
                println!("created user: {}", body.name);
                res.send(format!("created user: {}", body.name));
            }
            None => {
                res.status(400);
                res.send("invalid or missing JSON body");
            }
        },
    )
}
