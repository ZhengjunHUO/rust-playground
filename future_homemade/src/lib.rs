mod custom_future;
mod executor;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_future_and_block_on() {
        let f = custom_future::init_future(|| format!("From custom future"));
        let rslt = executor::block_on(f);
        assert_eq!(rslt, "From custom future".to_owned());
    }

    #[test]
    fn test_delay_future() {
        use std::time::{Duration, Instant};
        let mut tokio = executor::MiniTokio::new();
        let f = custom_future::DelayFuture(Instant::now() + Duration::from_micros(20));
        tokio.spawn(f);
        tokio.exec();
    }
}
