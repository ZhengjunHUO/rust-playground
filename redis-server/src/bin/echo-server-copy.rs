use tokio::io;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let ep = "127.0.0.1:8080";
    let listener = TcpListener::bind(ep).await?;
    println!("Server up at {} !", ep);

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            println!("Receive incoming request !");
            let (mut reader_half, mut writer_half) = socket.split();

            if let Err(e) = io::copy(&mut reader_half, &mut writer_half).await {
                eprintln!("Error occurred while copying: {}", e);
            }
            println!("Done !");
        });
    }
}
