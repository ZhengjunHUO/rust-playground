use warp::Filter;
use web_warp::filters::{show_all, update_votes};
use web_warp::handlers::error_handler;
use web_warp::models::init_demo_db;

#[tokio::main]
async fn main() {
    let candidates = init_demo_db();

    // Add or update candidate's vote
    // $ curl -X POST -H "Content-Type: application/json" -d '{"name": "huo","votes": 1}' 127.0.0.1:8000/vote
    let vote = update_votes(candidates.clone());

    // Retrieve all candidates' current stats
    // $ curl 127.0.0.1:8000/show
    let show = show_all(candidates.clone());

    warp::serve(vote.or(show).recover(error_handler))
        .run(([127, 0, 0, 1], 8000))
        .await;
}
