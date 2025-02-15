pub trait AsyncProcess<X, Y, Z> {
    fn spawn(&self, input: X) -> Result<Y, String>;
    fn retrieve(&self, key: Y) -> Result<Z, String>;
}

fn handle<T>(handle: T, input: i32) -> Result<i32, String>
where
    T: AsyncProcess<i32, String, i32>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;
    use mockall::predicate::*;

    mock! {
        S3Handler {}
        impl AsyncProcess<i32, String, i32> for S3Handler {
            fn spawn(&self, input: i32) -> Result<String, String>;
            fn retrieve(&self, key: String) -> Result<i32, String>;
        }
    }

    #[test]
    fn test_handle_ok() {
        let mut h = MockS3Handler::new();
        h.expect_spawn()
            .with(eq(9))
            .returning(|_| Ok(String::from("test_key")));
        h.expect_retrieve()
            .with(eq(String::from("test_key")))
            .returning(|_| Ok(8));

        let rslt = handle(h, 9);
        assert_eq!(rslt, Ok(80));
    }

    #[test]
    fn test_handle_ko() {
        // Arrange
        let mut h = MockS3Handler::new();
        h.expect_spawn()
            .with(eq(4))
            .returning(|_| Ok(String::from("test_key")));
        h.expect_retrieve()
            .with(eq(String::from("test_key")))
            .returning(|_| Ok(11));

        // Act
        let rslt = handle(h, 4);

        // Assert
        assert_eq!(rslt, Err(String::from("Too big.")));
    }
}

fn main() {}
