use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use warp::Filter;

#[derive(Deserialize, Serialize)]
struct Candidate {
    name: String,
    votes: u32,
}

type CandidateList = Arc<Mutex<HashMap<String, u32>>>;

#[tokio::main]
async fn main() {
    let candidates = Arc::new(Mutex::new(HashMap::from([
        (String::from("huo"), 0),
        (String::from("wang"), 0),
        (String::from("fufu"), 0),
    ])));

    let vote = warp::post()
        .and(warp::path!("vote"))
        .and(warp::body::content_length_limit(4096))
        .and(warp::body::json())
        .map(move |payload: Candidate| {
            update_candidate(&payload.name, payload.votes, candidates.clone())
        });

    warp::serve(vote).run(([127, 0, 0, 1], 8000)).await;
}

fn update_candidate(name: &str, votes: u32, cands: CandidateList) -> String {
    let mut guard = cands.lock().unwrap();
    guard
        .entry(name.to_string())
        .and_modify(|sum| *sum += votes)
        .or_insert(votes);
    println!("[DEBUG] {:?}", guard);
    format!("{} got {} vote(s) !", name, *guard.get(name).unwrap())
}
