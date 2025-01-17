#![feature(coroutines, coroutine_trait)]

use std::{
    ops::{Coroutine, CoroutineState},
    sync::{Arc, Mutex},
};

pub struct Vault {
    record: Arc<Mutex<u8>>,
    remain: u8,
}

impl Coroutine<()> for Vault {
    type Yield = ();
    type Return = ();

    fn resume(
        mut self: std::pin::Pin<&mut Self>,
        _arg: (),
    ) -> CoroutineState<Self::Yield, Self::Return> {
        match self.record.try_lock() {
            Ok(mut guard) => {
                *guard += 1;
            }
            Err(_) => return CoroutineState::Yielded(()),
        }

        self.remain -= 1;
        if self.remain == 0 {
            return CoroutineState::Complete(());
        }
        CoroutineState::Yielded(())
    }
}

#[cfg(test)]
mod tests {
    use std::{future::Future, pin::Pin, task::Poll};

    use super::*;

    // Simple wrapper func / interface / adapter
    fn check_yield(cor: &mut Vault) -> bool {
        match Pin::new(cor).resume(()) {
            CoroutineState::Yielded(_) => true,
            CoroutineState::Complete(_) => false,
        }
    }

    // Async interface
    impl Future for Vault {
        type Output = ();

        fn poll(
            mut self: Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<Self::Output> {
            match Pin::new(&mut self).resume(()) {
                CoroutineState::Yielded(_) => {
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }
                CoroutineState::Complete(_) => Poll::Ready(()),
            }
        }
    }

    #[test]
    fn simple_test() {
        let record = Arc::new(Mutex::new(0));
        let mut foo = Vault {
            record: record.clone(),
            remain: 2,
        };
        let mut bar = Vault {
            record: record.clone(),
            remain: 2,
        };

        {
            let guard = record.lock().unwrap();
            for _ in 0..3 {
                assert_eq!(check_yield(&mut foo), true);
                assert_eq!(check_yield(&mut bar), true);
            }
            assert_eq!(*guard, 0);
        }

        assert_eq!(check_yield(&mut foo), true);
        assert_eq!(*record.lock().unwrap(), 1);
        assert_eq!(check_yield(&mut bar), true);
        assert_eq!(*record.lock().unwrap(), 2);
        assert_eq!(check_yield(&mut foo), false);
        assert_eq!(*record.lock().unwrap(), 3);
        assert_eq!(check_yield(&mut bar), false);
        assert_eq!(*record.lock().unwrap(), 4);
    }

    #[tokio::test]
    async fn async_test() {
        let record = Arc::new(Mutex::new(0));
        let foo = Vault {
            record: record.clone(),
            remain: 2,
        };
        let bar = Vault {
            record: record.clone(),
            remain: 2,
        };
        let handle_foo = tokio::spawn(foo);
        let handle_bar = tokio::spawn(bar);
        handle_foo.await.unwrap();
        handle_bar.await.unwrap();
        assert_eq!(*record.lock().unwrap(), 4);
    }
}

fn main() {}
