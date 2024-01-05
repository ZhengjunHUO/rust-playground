use crate::models::init_demo_db;
use crate::models::{Candidate, CandidateList};
use warp::Filter;

pub fn all_routes() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let candidates = init_demo_db();

    // Add or update candidate's vote
    // $ curl -X POST -H "Content-Type: application/json" -d '{"name": "huo","votes": 1}' 127.0.0.1:8000/vote
    let vote = update_votes(candidates.clone());

    // Retrieve all candidates' current stats
    // $ curl 127.0.0.1:8000/show
    let show = show_all(candidates.clone());

    // Dummy api route to simulate an async remote call
    // $ curl 127.0.0.1:8000/dummy
    let dummy = get_dummy();

    vote.or(show).or(dummy)
}

fn with_candlist(
    list: CandidateList,
) -> impl Filter<Extract = (CandidateList,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || list.clone())
}

/// POST /vote with json body. eg. '{"name": "foo","votes": 1}'
fn update_votes(
    db: CandidateList,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path!("vote"))
        .and(warp::body::content_length_limit(4096))
        .and(warp::body::json())
        .and(with_candlist(db))
        .map(|payload: Candidate, candlist: CandidateList| {
            update_candidate(&payload.name, payload.votes, candlist)
        })
}

/// GET /show
fn show_all(
    db: CandidateList,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("show"))
        .and(with_candlist(db))
        .map(|candlist: CandidateList| print_all(candlist))
}

/// GET /dummy
fn get_dummy() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("dummy"))
        .then(|| dummy_handle_request())
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

async fn dummy_handle_request() -> Result<impl warp::Reply, std::convert::Infallible> {
    use std::{thread, time};
    thread::sleep(time::Duration::from_secs(3));
    let resp = String::from("Done");
    Ok(warp::reply::json(&resp))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::init_demo_db;

    use std::ops::Deref;

    #[tokio::test]
    async fn test_print_all() {
        let db = init_demo_db();
        let filter = show_all(db);
        let resp = warp::test::request().path("/show").reply(&filter).await;
        assert_eq!(resp.status(), 200);
        assert_eq!(resp.body().len(), 92);
    }

    #[tokio::test]
    async fn test_update_votes() {
        let db = init_demo_db();
        let filter = update_votes(db);
        let cand = Candidate {
            name: String::from("huo"),
            votes: 1,
        };
        let resp = warp::test::request()
            .method("POST")
            .json(&cand)
            .path("/vote")
            .reply(&filter)
            .await;
        assert_eq!(resp.status(), 200);
        assert_eq!(resp.body().deref(), b"huo got 1 vote(s) !\n");
    }
}
