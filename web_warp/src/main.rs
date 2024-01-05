use warp::Filter;
use web_warp::filters::all_routes;
use web_warp::handlers::error_handler;

#[tokio::main]
async fn main() {
    warp::serve(all_routes().recover(error_handler))
        .run(([127, 0, 0, 1], 8000))
        .await;
}
