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

    let cand_vote = candidates.clone();
    // curl -X POST -H "Content-Type: application/json" -d '{"name": "huo","votes": 1}' 127.0.0.1:8000/vote
    let vote = warp::post()
        .and(warp::path!("vote"))
        .and(warp::body::content_length_limit(4096))
        .and(warp::body::json())
        .and(warp::header("user-agent"))
        .map(move |payload: Candidate, agent: String| {
            println!("[DEBUG] Receive votes from agent {}", agent);
            update_candidate(&payload.name, payload.votes, cand_vote.clone())
        });

    let cand_show = candidates.clone();
    // curl 127.0.0.1:8000/show
    let show = warp::get()
        .and(warp::path!("show"))
        .map(move || print_all(cand_show.clone()));

    let routes = vote.or(show);

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn update_candidate(name: &str, votes: u32, cands: CandidateList) -> String {
    let mut guard = cands.lock().unwrap();
    guard
        .entry(name.to_string())
        .and_modify(|sum| *sum += votes)
        .or_insert(votes);
    println!("[DEBUG] {:?}", guard);
    format!("{} got {} vote(s) !\n", name, *guard.get(name).unwrap())
}

fn print_all(cands: CandidateList) -> String {
    let guard = cands.lock().unwrap();
    guard
        .iter()
        .map(|(name, sum)| format!("{} currently has {} vote(s) !\n", name, sum))
        .collect::<String>()
}
