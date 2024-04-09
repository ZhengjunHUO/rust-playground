use async_tasks::demo::DemoProject;
use async_tasks::exec::exec_async_tasks;

#[tokio::main]
async fn main() {
    let demo = DemoProject::new();
    exec_async_tasks::<DemoProject>(demo).await;
}
