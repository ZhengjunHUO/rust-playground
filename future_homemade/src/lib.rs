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
}
