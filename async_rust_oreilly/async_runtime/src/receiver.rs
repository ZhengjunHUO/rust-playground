use std::{
    io::{self, Read},
    net::TcpStream,
    sync::{Arc, Mutex},
    task::Poll,
};

pub struct CustomTcpReceiver {
    pub stream: Arc<Mutex<TcpStream>>,
    pub buf: Vec<u8>,
}

impl Future for CustomTcpReceiver {
    type Output = io::Result<Vec<u8>>;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
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

        let mut temp_buf = [0; 1024];
        match stream.read(&mut temp_buf) {
            Ok(0) => Poll::Ready(Ok(self.buf.to_vec())),
            Ok(n) => {
                std::mem::drop(stream);
                self.buf.extend_from_slice(&temp_buf[..n]);
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            Err(e) => Poll::Ready(Err(e)),
        }
    }
}
