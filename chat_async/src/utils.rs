use async_std::prelude::*;
use async_std::io;
use serde::Serialize;
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
