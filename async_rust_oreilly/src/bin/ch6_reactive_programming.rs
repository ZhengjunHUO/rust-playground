use async_rust_oreilly::observers::{Display, HeatLoss, Heater};

#[tokio::main]
async fn main() {
    let handlers = vec![
        tokio::spawn(Display::new()),
        tokio::spawn(Heater::new()),
        tokio::spawn(HeatLoss::new()),
    ];
    for handler in handlers {
        handler.await.unwrap();
    }
}
