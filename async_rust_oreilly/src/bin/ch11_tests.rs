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
    use std::sync::Arc;

    use super::*;
    use mockall::mock;
    use mockall::predicate::*;
    use tokio::sync::Mutex;
    use tokio::time::{sleep, timeout, Duration};

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

    #[tokio::test]
    async fn test_deadlock_detect() {
        let lock0 = Arc::new(Mutex::new(vec![0]));
        let lock0_clone = lock0.clone();
        let lock1 = Arc::new(Mutex::new(vec![1]));
        let lock1_clone = lock0.clone();

        let task1 = tokio::spawn(async move {
            let guard0 = lock0.lock().await;
            sleep(Duration::from_millis(200)).await;
            let guard1 = lock1.lock().await;
            println!("Read from locks: {:?}; {:?}", guard0, guard1);
        });

        let task2 = tokio::spawn(async move {
            let guard1 = lock1_clone.lock().await;
            sleep(Duration::from_millis(200)).await;
            let guard0 = lock0_clone.lock().await;
            println!("Read from locks: {:?}; {:?}", guard1, guard0);
        });

        let rslt = timeout(Duration::from_secs(3), async {
            let _ = task2.await;
            let _ = task1.await;
        })
        .await;

        assert!(rslt.is_ok(), "Deadlock found !");
    }
}

fn main() {}
