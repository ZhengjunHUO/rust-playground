use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Error, Read};
use std::path::Path;
use std::sync::mpsc;
use threadpool::ThreadPool;
use walkdir::WalkDir;

fn is_zip(path: &Path) -> bool {
    match path.extension() {
        Some(ext) if ext.to_string_lossy().to_lowercase() == "zip" => true,
        _ => false,
    }
}

fn calc_digest<P: AsRef<Path>>(path: P) -> Result<(Digest, P), Error> {
    let mut reader = BufReader::new(File::open(&path)?);
    let mut ctx = Context::new(&SHA256);
    let mut buf = [0; 2048];

    loop {
        let count = reader.read(&mut buf)?;
        if count == 0 {
            break;
        }
        ctx.update(&buf[..count]);
    }

    Ok((ctx.finish(), path))
}

fn main() {
    let path = std::env::args().nth(1).expect("Wait for a dir path");

    let pool = ThreadPool::new(3);
    let (tx, rx) = mpsc::channel();

    for item in WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|x| x.ok())
        .filter(|x| !x.path().is_dir() && is_zip(x.path()))
    {
        let p = item.path().to_owned();
        let sender = tx.clone();
        pool.execute(move || {
            let digest = calc_digest(p).expect("Digest calculation failed");
            sender.send(digest).expect("Failed to send result");
        });
    }

    drop(tx);

    while let Ok((sha, path)) = rx.recv() {
        println!("{:?}: {:?}", path, sha);
    }
}
