use std::net::{TcpListener, TcpStream};

mod handlers;
mod http_request;
mod http_response;
mod method;
mod route;
mod router;

use http_request::HttpRequest;
use http_response::HttpResponse;
use route::Route;
use router::{build_routes, find_route};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let routes = build_routes();

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => stream,
            Err(e) => {
                eprintln!("Failed to accept connection: {e}");
                continue;
            }
        };

        handle_connection(stream, &routes);
    }
}

/// Reads an incoming HTTP request from `stream`, dispatches it to the
/// matching route in `routes`, and writes the resulting response back.
///
/// Unmatched requests get a `404` response instead of a handler call.
fn handle_connection(stream: TcpStream, routes: &[Route]) {
    let mut request = match HttpRequest::from_stream(&stream) {
        Ok(Some(request)) => request,
        Ok(None) => return,
        Err(e) => {
            eprintln!("Failed to read request from connection: {e}");
            return;
        }
    };

    let mut response = HttpResponse::default();
    response.status(200);

    match find_route(routes, &request.method, &request.path) {
        Some((route, path_params)) => {
            request.params.extend(path_params);
            (route.handler)(&request, &mut response);
        }
        None => {
            response.status(404);
            response.send("not found");
        }
    }

    if let Err(e) = response.write_to(&stream) {
        eprintln!("Failed to write response to connection: {e}");
    }
}
