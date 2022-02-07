use hyper::{Body, Request, Response};
use hyper::service::{make_service_fn, service_fn};
use crate::BoxedResult;
use crate::constants::WebResponse;

async fn hello_world(req: Request<Body>) -> WebResponse {
    match (req.method(), req.uri().path()) {
        (&hyper::Method::GET, "/") => Ok(Response::new(Body::from("Hello, World!"))),
        _ => Ok(Response::new(Body::from("Hello, World!"))),
    }
}



pub async fn start() -> BoxedResult<()> {
    let addr = ([127, 0, 0, 1], 7329).into();

    let service = make_service_fn(|_| async {
        Ok::<_, hyper::Error>(service_fn(hello_world))
    });

    let server = hyper::Server::bind(&addr).serve(service);
    println!("Listening on http://{}", addr);
    server.await?;
    Ok(())
}