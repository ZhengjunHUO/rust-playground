pub mod executor;
pub mod myfuture;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_myfuture() {
        let f = myfuture::spawn_blocking(|| "Rustacean rocks!".to_string());

        let rslt = executor::block_on(f);
        assert_eq!(rslt, "Rustacean rocks!".to_string());
    }
}
