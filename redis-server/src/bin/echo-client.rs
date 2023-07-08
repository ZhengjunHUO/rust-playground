use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = TcpStream::connect("127.0.0.1:8080").await?;
    let (mut reader_half, mut writer_half) = socket.into_split();

    tokio::spawn(async move {
        writer_half.write_all(b"Rust\r\n").await?;
        writer_half.write_all(b"Rocks\r\n").await?;
        println!("Data sent.");
        Ok::<_, io::Error>(())
    });

    let mut buf = vec![0, 64];
    loop {
        let n = reader_half.read(&mut buf).await?;
        println!("Read {} bytes from server", n);
        if n == 0 {
            break;
        }
        println!("Reply from server: {:?}", &buf[..n]);
    }

    Ok(())
}
