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
/// # Arguments
/// - `stream` — the accepted client connection to read the request from and
///   write the response to.
/// - `routes` — the routing table to dispatch the parsed request against.
///
/// Returns nothing; the response (success, `400` on an unreadable request,
/// or `404` on no matching route) is written directly to `stream`.
fn handle_connection(stream: TcpStream, routes: &[Route]) {
    let mut response = HttpResponse::default();
    response.status(200);

    let request = match HttpRequest::from_stream(&stream) {
        Ok(Some(request)) => request,
        Ok(None) => {
            response.status(400);
            response.send("empty request");
            return write_response(&response, &stream);
        }
        Err(e) => {
            eprintln!("Failed to read request from connection: {e}");
            response.status(400);
            response.send(e.to_string());
            return write_response(&response, &stream);
        }
    };

    dispatch(routes, request, &mut response);
    write_response(&response, &stream);
}

/// Writes `response` back to `stream`, logging (without panicking) if the
/// write fails.
///
/// # Arguments
/// - `response` — the response to serialize as raw HTTP bytes.
/// - `stream` — the connection to write those bytes to.
///
/// Returns nothing; write failures are logged to stderr, not propagated.
fn write_response(response: &HttpResponse, stream: &TcpStream) {
    if let Err(e) = response.write_to(stream) {
        eprintln!("Failed to write response to connection: {e}");
    }
}

/// Routes `request` to the matching handler in `routes`, populating
/// `response` in place. Unmatched requests get a `404` response instead of
/// a handler call.
///
/// # Arguments
/// - `routes` — the routing table to search for a match.
/// - `request` — the incoming request; gains its path parameters (if any)
///   once a route matches.
/// - `response` — the response to populate; starts however the caller left
///   it (`handle_connection` pre-sets `200`) and is overwritten by whichever
///   handler runs, or set to `404` if nothing matches.
///
/// Returns nothing; the result is written into `response`.
fn dispatch(routes: &[Route], mut request: HttpRequest, response: &mut HttpResponse) {
    match find_route(routes, &request.method, &request.path) {
        Ok((route, path_params)) => {
            request.set_path_params(path_params);
            (route.handler)(&request, response);
        }
        Err(error) => {
            response.status(404);
            response.send(error);
        }
    }
}

#[cfg(test)]
mod tests;
