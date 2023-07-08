use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    /* Read to fill the buffer
    let mut f = File::open("foo.txt").await?;
    let mut buffer = [0; 15];

    let n = f.read(&mut buffer[..]).await?;
    println!("Read {} bytes from the text: {:?}", n, &buffer[..n]);
    */

    /* Read all from file, and write everything to another file
    let mut f = File::open("foo.txt").await?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).await?;
    println!("Read {} bytes from the text: {:?}", buf.len(), buf);

    let mut fw = File::create("bar.txt").await?;
    fw.write_all(&buf[..]).await?;
    println!("Written to somewhere else.");
    */

    let mut reader: &[u8] = b"Rust rocks";
    let mut writer = File::create("bar.txt").await?;

    io::copy(&mut reader, &mut writer).await?;
    println!("Copy done !");
    Ok(())
}
