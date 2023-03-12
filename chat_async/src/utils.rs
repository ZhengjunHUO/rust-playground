use async_std::io;
use async_std::prelude::*;
use serde::{de::DeserializeOwned, Serialize};
use std::marker::Unpin;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T> = std::result::Result<T, Error>;

pub async fn marshal_and_send<W, S>(dest: &mut W, req: &S) -> Result<()>
where
    W: io::Write + Unpin,
    S: Serialize,
{
    let mut s = serde_json::to_string(&req)?;
    s.push('\n');
    dest.write_all(s.as_bytes()).await?;
    Ok(())
}

pub async fn recv_and_unmarshal<R, D>(orig: R) -> impl Stream<Item = Result<D>>
where
    R: io::BufRead + Unpin,
    D: DeserializeOwned,
{
    orig.lines().map(|line| -> Result<D> {
        let rslt = line?;
        let d = serde_json::from_str::<D>(&rslt)?;
        Ok(d)
    })
}
