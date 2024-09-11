use http_body_util::Full;
use hyper::body::Bytes;
use hyper::service::service_fn;
use hyper::{body, Request, Response};
use hyper_tower_axum_server_example::HyperToTowerService;
use std::convert::Infallible;
use std::net::SocketAddr;
use tower::make;

#[tokio::main]
async fn main() {
    let service = service_fn(|_req: Request<body::Incoming>| async move {
        Result::<_, Infallible>::Ok(Response::new(Full::<Bytes>::from("Hello World")))
    });
    let tower_service = HyperToTowerService::new(service);
    let make_service = make::Shared::new(tower_service);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum_server::bind(addr).serve(make_service).await.unwrap();
}
