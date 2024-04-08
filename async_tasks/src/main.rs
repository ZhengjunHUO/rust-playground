use async_tasks::demo::DemoProject;
use async_tasks::exec::exec_async_tasks;

#[tokio::main]
async fn main() {
    exec_async_tasks::<DemoProject>().await;
}
