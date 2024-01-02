use crate::models::{Candidate, CandidateList};
use warp::Filter;

fn with_candlist(
    list: CandidateList,
) -> impl Filter<Extract = (CandidateList,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || list.clone())
}

/// POST /vote with json body. eg. '{"name": "foo","votes": 1}'
pub fn update_votes(
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
pub fn show_all(
    db: CandidateList,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("show"))
        .and(with_candlist(db))
        .map(|candlist: CandidateList| print_all(candlist))
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
