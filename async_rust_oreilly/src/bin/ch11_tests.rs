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
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::task::Poll;

    use super::*;
    use mockall::mock;
    use mockall::predicate::*;
    use mockito::Matcher;
    use tokio::runtime::Builder;
    use tokio::sync::{mpsc, Mutex};
    use tokio::time::{sleep, timeout, Duration};
    use tokio_test::assert_pending;
    use tokio_test::task::spawn;

    static COUNTER_SINGLE: AtomicUsize = AtomicUsize::new(0);
    static COUNTER_MULTI: AtomicUsize = AtomicUsize::new(0);
    static COUNTER_MULTI_SLEEP: AtomicUsize = AtomicUsize::new(0);

    async fn non_atomic_add() {
        let val = COUNTER_SINGLE.load(Ordering::SeqCst);
        COUNTER_SINGLE.store(val + 1, Ordering::SeqCst);
    }

    async fn non_atomic_add_multi() {
        let val = COUNTER_MULTI.load(Ordering::SeqCst);
        COUNTER_MULTI.store(val + 1, Ordering::SeqCst);
    }

    async fn non_atomic_add_multi_sleep() {
        let val = COUNTER_MULTI_SLEEP.load(Ordering::SeqCst);
        sleep(Duration::from_millis(200)).await;
        COUNTER_MULTI_SLEEP.store(val + 1, Ordering::SeqCst);
    }

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
    async fn test_deadlock_detect_fail() {
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

        let rslt = timeout(Duration::from_secs(1), async {
            let _ = task2.await;
            let _ = task1.await;
        })
        .await;

        assert!(rslt.is_ok(), "Deadlock found !");
    }

    #[test]
    fn test_race_condition_single_thread_ok() {
        let runtime = Builder::new_current_thread().enable_all().build().unwrap();
        let mut handles = vec![];
        let num = 10000;
        for _ in 0..num {
            let h = runtime.spawn(non_atomic_add());
            handles.push(h);
        }
        for h in handles {
            runtime.block_on(h).unwrap();
        }
        // Will pass, because only 1 thread is running.
        assert_eq!(
            COUNTER_SINGLE.load(Ordering::SeqCst),
            num,
            "Race condition found !"
        );
    }

    #[test]
    fn test_race_condition_multi_thread_fail() {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let mut handles = vec![];
        let num = 10000;
        for _ in 0..num {
            let h = runtime.spawn(non_atomic_add_multi());
            handles.push(h);
        }
        for h in handles {
            runtime.block_on(h).unwrap();
        }
        // Will fail, COUNTER_MULTI is less than num
        assert_eq!(
            COUNTER_MULTI.load(Ordering::SeqCst),
            num,
            "Race condition found !"
        );
    }

    #[test]
    fn test_race_condition_multi_thread_with_sleep_fail() {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let mut handles = vec![];
        let num = 10000;
        for _ in 0..num {
            let h = runtime.spawn(non_atomic_add_multi_sleep());
            handles.push(h);
        }
        for h in handles {
            runtime.block_on(h).unwrap();
        }
        // Will fail, COUNTER_MULTI will be 1, each task will read 0 before sleep, and write 1 back
        assert_eq!(
            COUNTER_MULTI_SLEEP.load(Ordering::SeqCst),
            num,
            "Race condition found !"
        );
    }

    #[test]
    fn test_channel_capacity_fail() {
        let runtime = Builder::new_current_thread().enable_all().build().unwrap();
        let (tx, _rx) = mpsc::channel::<i32>(5);
        let h = runtime.spawn(async move {
            for i in 0..10 {
                tx.send(i).await.unwrap();
            }
        });

        let rslt = runtime.block_on(async {
            timeout(Duration::from_secs(1), async {
                h.await.unwrap();
            })
            .await
        });

        assert!(rslt.is_ok(), "Channel's buffer is full !");
    }

    #[test]
    fn test_channel_capacity_success() {
        let runtime = Builder::new_current_thread().enable_all().build().unwrap();
        let (tx, mut rx) = mpsc::channel::<i32>(5);
        let h = runtime.spawn(async move {
            for i in 0..10 {
                tx.send(i).await.unwrap();
            }
        });

        runtime.spawn(async move {
            let mut i = 0;
            while let Some(data) = rx.recv().await {
                assert_eq!(i, data);
                println!("Recv data: {}", data);
                i += 1;
            }
        });

        let rslt = runtime.block_on(async {
            timeout(Duration::from_secs(1), async {
                h.await.unwrap();
            })
            .await
        });

        assert!(rslt.is_ok(), "Channel's buffer is full !");
    }

    #[test]
    fn test_networking() {
        let mut server = mockito::Server::new();
        let url = server.url();

        let mock = server
            .mock("GET", "/req")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("param1".into(), "val1".into()),
                Matcher::UrlEncoded("param2".into(), "val2".into()),
            ]))
            .with_status(201)
            .with_body("Rustacean")
            .expect(5)
            .create();

        let runtime = Builder::new_current_thread().enable_all().build().unwrap();
        let mut handles = vec![];

        for _ in 0..5 {
            let url_clone = url.clone();
            handles.push(runtime.spawn(async move {
                let client = reqwest::Client::new();
                client
                    .get(format!("{}/req?param1=val1&param2=val2", url_clone))
                    .send()
                    .await
                    .unwrap()
            }));
        }

        for h in handles {
            runtime.block_on(h).unwrap();
        }

        mock.assert();
    }

    async fn incr_mutex(lock: Arc<Mutex<i32>>) {
        let mut guard = lock.lock().await;
        *guard += 1;
        sleep(Duration::from_millis(1)).await;
    }

    #[tokio::test]
    async fn test_future() {
        let lock = Arc::new(Mutex::new(0));
        let lock0 = lock.clone();
        let lock1 = lock.clone();

        let mut h1 = spawn(incr_mutex(lock0));
        let mut h2 = spawn(incr_mutex(lock1));

        // h1 will acquire the lock, although the state is Pending
        assert_pending!(h1.poll());
        assert_pending!(h2.poll());

        // h2 will always Pending since h1 got the lock
        for _ in 0..10 {
            assert_pending!(h2.poll());
            sleep(Duration::from_millis(1)).await;
        }

        // h1 should be ready now
        assert_eq!(h1.poll(), Poll::Ready(()));
        // h2 still Pending since h1 is not dropped yet
        assert_pending!(h2.poll());

        drop(h1);
        sleep(Duration::from_millis(1)).await;
        // h1 dropped, h2 will acquire the lock and get Ready
        assert_eq!(h2.poll(), Poll::Ready(()));

        let guard = lock.lock().await;
        assert_eq!(*guard, 2);
    }
}

fn main() {}
