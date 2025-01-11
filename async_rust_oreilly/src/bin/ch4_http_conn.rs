use std::{
    net::{TcpStream, ToSocketAddrs},
    pin::Pin,
    task::{Context, Poll},
};

use anyhow::{bail, Context as _, Error, Ok};
use async_native_tls::TlsStream;
use async_rust_oreilly::task_spawn;
use hyper::{Client, Request, Uri};
use smol::{future, Async};

struct MyExecutor;
impl<F: std::future::Future + Send + 'static> hyper::rt::Executor<F> for MyExecutor {
    fn execute(&self, fut: F) {
        task_spawn!(async {
            println!("Try sending req ...");
            fut.await;
            println!("Done");
        })
        .detach();
    }
}

enum MyStream {
    PlainText(Async<TcpStream>),
    Ciphered(TlsStream<Async<TcpStream>>),
}

#[derive(Clone)]
struct MyConnector;
// The Service trait defines the future for the connection.
impl hyper::service::Service<Uri> for MyConnector {
    type Response = MyStream;
    type Error = Error;
    type Future =
        Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    // The poll_ready function needs to return Ok before we can use call
    fn call(&mut self, req: Uri) -> Self::Future {
        Box::pin(async move {
            let host = req.host().context("Error parsing host")?;
            match req.scheme_str() {
                Some("http") => {
                    let socket = {
                        let addr = host.to_string();
                        let port = req.port_u16().unwrap_or(80);
                        smol::unblock(move || (addr.as_str(), port).to_socket_addrs())
                            .await?
                            .next()
                            .context("Error resolving socket addr")?
                    };
                    let tcp_stream = Async::<TcpStream>::connect(socket).await?;
                    Ok(MyStream::PlainText(tcp_stream))
                }
                Some("https") => {
                    let socket = {
                        let addr = host.to_string();
                        let port = req.port_u16().unwrap_or(443);
                        smol::unblock(move || (addr.as_str(), port).to_socket_addrs())
                            .await?
                            .next()
                            .context("Error resolving socket addr")?
                    };
                    let tcp_stream = Async::<TcpStream>::connect(socket).await?;
                    let stream = async_native_tls::connect(host, tcp_stream).await?;
                    Ok(MyStream::Ciphered(stream))
                }
                scheme => bail!("Unsupported scheme {:?}", scheme),
            }
        })
    }
}

fn main() {
    let uri: Uri = "http://www.rust-lang.org".parse().unwrap();
    let req = Request::builder()
        .method("GET")
        .uri(uri)
        .header("User-Agent", "hyper/0.14.2")
        .header("Accept", "text/html")
        .body(hyper::Body::empty())
        .unwrap();

    let future = async {
        let client = Client::new();
        client.request(req).await.unwrap()
    };
    let handle = task_spawn!(future);
    let resp = future::block_on(handle);
    println!("Get response: {}", resp.status());
}
