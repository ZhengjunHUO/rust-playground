use std::iter::from_fn;
use std::ops::Add;

pub trait Positive {
    // associated consts, declaration only
    const ZERO: Self;
    const ONE: Self;
}

impl Positive for u32 {
    const ZERO: u32 = 0;
    const ONE: u32 = 1;
}

impl Positive for u64 {
    const ZERO: u64 = 0;
    const ONE: u64 = 1;
}

impl Positive for f32 {
    const ZERO: f32 = 0.0;
    const ONE: f32 = 1.0;
}

pub fn fib<T>(n: usize) -> T
where
    T: Positive + Add<Output = T>,
{
    match n {
        0 => T::ZERO,
        1 => T::ONE,
        n => fib::<T>(n - 1) + fib::<T>(n - 2),
    }
}

pub fn fib_list(n: usize) -> Vec<u32> {
    let mut binome = (0, 1);
    from_fn(move || {
        binome = (binome.1, binome.0 + binome.1);
        Some(binome.0)
    })
    .take(n)
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        assert_eq!(fib::<u32>(8), 21);
        assert_eq!(fib::<u64>(10), 55);
        assert_eq!(fib::<f32>(9), 34.0);
        assert_eq!(fib_list(5), vec![1, 1, 2, 3, 5]);
    }
}
