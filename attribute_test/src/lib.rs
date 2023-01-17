#[cfg(test)]
mod tests {
    #[test]
    #[allow(unconditional_panic)]
    #[should_panic(expected="divide by zero")]
    fn test_trigger_panic() {
        let _ = 1 / 0;
    }
}
