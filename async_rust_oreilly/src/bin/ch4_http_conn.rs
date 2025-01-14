use std::{
    net::{TcpStream, ToSocketAddrs},
    pin::Pin,
    task::{Context, Poll},
};

use anyhow::{bail, Context as _, Error, Result};
use async_native_tls::TlsStream;
use async_rust_oreilly::{task_spawn, tasks::Runtime};
use futures_lite::{AsyncRead, AsyncWrite};
use http::Uri;
use hyper::{Body, Client, Request, Response};
use smol::{io, prelude::*, Async};

struct MyExecutor;
impl<F: Future + Send + 'static> hyper::rt::Executor<F> for MyExecutor {
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

impl tokio::io::AsyncRead for MyStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        match &mut *self {
            MyStream::PlainText(s) => Pin::new(s)
                .poll_read(cx, buf.initialize_unfilled())
                .map_ok(|size| buf.advance(size)),
            MyStream::Ciphered(s) => Pin::new(s)
                .poll_read(cx, buf.initialize_unfilled())
                .map_ok(|size| buf.advance(size)),
        }
    }
}

impl tokio::io::AsyncWrite for MyStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        match &mut *self {
            MyStream::PlainText(s) => Pin::new(s).poll_write(cx, buf),
            MyStream::Ciphered(s) => Pin::new(s).poll_write(cx, buf),
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match &mut *self {
            MyStream::PlainText(s) => Pin::new(s).poll_flush(cx),
            MyStream::Ciphered(s) => Pin::new(s).poll_flush(cx),
        }
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match &mut *self {
            MyStream::PlainText(s) => {
                s.get_ref().shutdown(std::net::Shutdown::Write)?;
                Poll::Ready(Ok(()))
            }
            MyStream::Ciphered(s) => Pin::new(s).poll_close(cx),
        }
    }
}

impl hyper::client::connect::Connection for MyStream {
    fn connected(&self) -> hyper::client::connect::Connected {
        hyper::client::connect::Connected::new()
    }
}

#[derive(Clone)]
struct MyConnector;
// The Service trait defines the future for the connection.
impl hyper::service::Service<Uri> for MyConnector {
    type Response = MyStream;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
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

async fn fetch(req: Request<Body>) -> Result<Response<Body>> {
    println!("Inside fetch");
    Ok(Client::builder()
        .executor(MyExecutor)
        .build::<_, Body>(MyConnector)
        .request(req)
        .await?)
}

fn main() {
    Runtime::new()
        .with_std_chan_num(1)
        .with_premium_chan_num(2)
        .run();

    let future = async {
        // let uri: Uri = "http://www.rust-lang.org".parse().unwrap();
        // let req = Request::builder()
        //     .method("GET")
        //     .uri(uri)
        //     .header("User-Agent", "hyper/0.14.2")
        //     .header("Accept", "text/html")
        //     .body(hyper::Body::empty())
        //     .unwrap();

        let req = Request::get("http://www.rust-lang.org")
            .body(Body::empty())
            .unwrap();
        let resp = fetch(req).await.unwrap();
        let bytes = hyper::body::to_bytes(resp.into_body()).await.unwrap();
        let rslt = String::from_utf8(bytes.to_vec()).unwrap();
        println!("Got resp: {}", rslt);
    };

    println!("[main] Before spawn task");
    let handle = task_spawn!(future);
    smol::future::block_on(handle);
    println!("[main] Done");
}
