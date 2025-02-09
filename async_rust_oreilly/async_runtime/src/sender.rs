use std::{
    io::{self, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
    task::Poll,
};

pub struct CustomTcpSender {
    pub stream: Arc<Mutex<TcpStream>>,
    pub buf: Vec<u8>,
}

impl Future for CustomTcpSender {
    type Output = io::Result<()>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut stream = match self.stream.try_lock() {
            Ok(stream) => stream,
            Err(_) => {
                cx.waker().wake_by_ref();
                return Poll::Pending;
            }
        };

        stream.set_nonblocking(true)?;
        match stream.write_all(&self.buf) {
            Ok(_) => Poll::Ready(Ok(())),
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            Err(e) => Poll::Ready(Err(e)),
        }
    }
}
