use serde::Deserialize;

use crate::http_request::HttpRequest;
use crate::http_response::HttpResponse;
use crate::route::RouteHandler;

/// Handles the default `/` route, confirming the server is running.
pub fn health() -> RouteHandler {
    Box::new(|_req: &HttpRequest, res: &mut HttpResponse| {
        res.send("Service is live");
    })
}

/// Handles `GET /user?id=<id>`, reading `id` from the query string.
pub fn get_user() -> RouteHandler {
    Box::new(
        |req: &HttpRequest, res: &mut HttpResponse| match req.params.get("id") {
            Some(id) => {
                println!("User id =>{id}");
                res.send(format!("location id: {id}"))
            }
            None => {
                res.status(400);
                res.send("missing \"id\" query param");
            }
        },
    )
}

/// Handles `GET /location/:id`, reading `id` from the path.
pub fn get_location() -> RouteHandler {
    Box::new(
        |req: &HttpRequest, res: &mut HttpResponse| match req.params.get("id") {
            Some(id) => {
                println!("User id =>{id}");
                res.send(format!("location id: {id}"))
            }
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
