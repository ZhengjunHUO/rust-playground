use std::{future::Future, pin::Pin};

trait Echo {
    fn echo(&self) -> String;
}

struct Hello;
impl Echo for Hello {
    fn echo(&self) -> String {
        String::from("Hello Rust !")
    }
}

struct EchoWrapper<T> {
    echoer: T,
}

impl<T> EchoWrapper<T>
where
    T: Echo,
{
    fn echo(&self) -> String {
        let mut rslt = String::from("Inside wrapper: ");
        let temp = self.echoer.echo();
        rslt.push_str(&temp);
        rslt
    }
}

trait Logger {
    fn log(&self);
}

/*
struct LoggerWrapper<F: Future + Logger> {
    inner: F,
}

impl<F: Future + Logger> Future for LoggerWrapper<F> {
    type Output = F::Output;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let inner = unsafe { self.map_unchecked_mut(|s| &mut s.inner) };
        inner.log();
        inner.poll(cx)
    }
}
*/

struct LoggerWrapper<F: Future + Logger> {
    inner: Pin<Box<F>>,
}

impl<F: Future + Logger> Future for LoggerWrapper<F> {
    type Output = F::Output;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let inner = self.get_mut().inner.as_mut();
        inner.log();
        inner.poll(cx)
    }
}

impl<F: Future> Logger for F {
    fn log(&self) {
        println!("Future get polled.");
    }
}

async fn demo_future() -> String {
    "Exec future done.".to_string()
}

#[tokio::main]
async fn main() {
    #[cfg(feature = "decorator_test")]
    let e: EchoWrapper<Hello> = EchoWrapper { echoer: Hello };
    #[cfg(not(feature = "decorator_test"))]
    let e = Hello;
    println!("{}", e.echo());

    /* 输出为：
    Future get polled.
    Result: Exec future done.
     */
    let wrapped_future = LoggerWrapper {
        inner: Box::pin(demo_future()),
    };
    println!("Result: {}", wrapped_future.await);
}
