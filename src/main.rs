use hyper::{Body, Request, Response, Server, Method};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Define the address and port for the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle_request)) });
    
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // Match on the request method type [GET, POST, etc]
    // small change
    match req.method() {
        &Method::GET => {      
            match req.uri().path() {
                "/" => Ok(Response::new(Body::from("Welcome to the simple HTTP server!"))),
                "/hello" => Ok(Response::new(Body::from("Hello, World!"))),
                "/goodbye" => Ok(Response::new(Body::from("Goodbye, World!"))),
                _ => Ok(Response::new(Body::from("404 Not Found"))),
            }
        },
        &Method::POST => {
            Ok(Response::new(Body::from("Hello from POST")))
        },
        _ => {
            Ok(Response::new(Body::from("Method not Allowed...")))
        },
    }
}

