use std::future::Future;

pub trait SyncProcess<X, Y, Z> {
    fn spawn(&self, input: X) -> Result<Y, String>;
    fn retrieve(&self, key: Y) -> Result<Z, String>;
}

pub fn handle_sync<T>(handle: T, input: i32) -> Result<i32, String>
where
    T: SyncProcess<i32, String, i32>,
{
    let key = handle.spawn(input)?;
    println!("Key received, retrieve answer");
    let rslt = handle.retrieve(key)?;
    if rslt > 10 {
        return Err(String::from("Too big."));
    }
    if rslt == 8 {
        return Ok(rslt * 10);
    }
    Ok(rslt * 3)
}

pub trait AsyncProcess<Y, Z> {
    fn retrieve(&self, key: Y) -> impl Future<Output = Result<Z, String>>;
}

pub async fn handle_async<T>(handle: T, input: i32) -> Result<i32, String>
where
    T: AsyncProcess<i32, i32>,
{
    let rslt = handle.retrieve(input).await?;
    if rslt > 10 {
        return Err(String::from("Too big."));
    }
    if rslt == 8 {
        return Ok(rslt * 10);
    }
    Ok(rslt * 3)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;
    use mockall::predicate::*;

    mock! {
        S3Handler {}
        impl SyncProcess<i32, String, i32> for S3Handler {
            fn spawn(&self, input: i32) -> Result<String, String>;
            fn retrieve(&self, key: String) -> Result<i32, String>;
        }
    }

    mock! {
        DBHandler {}
        impl AsyncProcess<i32, i32> for DBHandler {
            fn retrieve(&self, key: i32) -> impl Future<Output = Result<i32, String>>;
        }
    }

    #[test]
    fn test_handle_sync_ok() {
        let mut h = MockS3Handler::new();
        h.expect_spawn()
            .with(eq(9))
            .returning(|_| Ok(String::from("test_key")));
        h.expect_retrieve()
            .with(eq(String::from("test_key")))
            .returning(|_| Ok(8));

        let rslt = handle_sync(h, 9);
        assert_eq!(rslt, Ok(80));
    }

    #[test]
    fn test_handle_sync_ko() {
        // Arrange
        let mut h = MockS3Handler::new();
        h.expect_spawn()
            .with(eq(4))
            .returning(|_| Ok(String::from("test_key")));
        h.expect_retrieve()
            .with(eq(String::from("test_key")))
            .returning(|_| Ok(11));

        // Act
        let rslt = handle_sync(h, 4);

        // Assert
        assert_eq!(rslt, Err(String::from("Too big.")));
    }

    #[test]
    fn test_handle_async_ok() {
        let mut h = MockDBHandler::new();
        h.expect_retrieve()
            .with(eq(2))
            .returning(|_| Box::pin(async { Ok(9) }));
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let rslt = runtime.block_on(handle_async(h, 2));
        assert_eq!(rslt, Ok(27));
    }

    #[test]
    fn test_handle_async_ko() {
        let mut h = MockDBHandler::new();
        h.expect_retrieve()
            .with(eq(5))
            .returning(|_| Box::pin(async { Ok(20) }));

        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let rslt = runtime.block_on(handle_async(h, 5));
        assert_eq!(rslt, Err(String::from("Too big.")));
    }
}

fn main() {}
