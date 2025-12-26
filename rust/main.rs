use std::{convert::Infallible, net::SocketAddr};

use http_body_util::Full;
use hyper::{
  Request, Response,
  body::{Bytes, Incoming},
  server::conn::http1::Builder,
  service::service_fn,
};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

async fn router(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
  match req.uri().path() {
    "/" => Ok(Response::new(Full::new(Bytes::from(
      "Welcome to the home page!",
    )))),
    "/health" => Ok(Response::new(Full::new(Bytes::from("OK")))),
    _ => Ok(
      Response::builder()
        .status(404)
        .body(Full::new(Bytes::from("Not Found")))
        .unwrap(),
    ),
  }
}

#[tokio::main]
async fn main() -> () {
  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  let listener = TcpListener::bind(addr).await.unwrap();

  loop {
    let (stream, _) = listener.accept().await.unwrap();

    let io = TokioIo::new(stream);

    tokio::task::spawn(async move {
      if let Err(err) = Builder::new()
        .serve_connection(io, service_fn(router))
        .await
      {
        eprintln!("Error serving connection: {:?}", err);
      }
    });
  }
}
