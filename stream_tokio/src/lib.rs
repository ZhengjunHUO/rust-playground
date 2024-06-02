mod custom_future;
mod custom_stream;

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_stream::StreamExt;

    #[tokio::test]
    async fn test_custom_stream() {
        let mut count = 0;
        let mut custom_stream = custom_stream::CustomStream::new();
        while let Some(_) = custom_stream.next().await {
            count += 1;
        }

        assert_eq!(count, 5)
    }

    #[tokio::test]
    async fn test_custom_stream_macro() {
        let mut count = 0;
        let stream = custom_stream::new_custom_stream();
        tokio::pin!(stream);

        while let Some(_) = stream.next().await {
            count += 1;
        }

        assert_eq!(count, 5)
    }
}
