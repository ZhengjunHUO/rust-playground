use web_warp::filters::all_routes_handled;

#[tokio::main]
async fn main() {
    warp::serve(all_routes_handled())
        .run(([127, 0, 0, 1], 8000))
        .await;
}
