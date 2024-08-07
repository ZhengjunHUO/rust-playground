use crate::handlers::{
    auth_check, check_status, dummy_handle_request, dummy_submit_handle_request, error_handler, print_all, update_candidate, with_candlist
};
use crate::models::{init_demo_db, Candidate, CandidateList};
use warp::Filter;

pub fn all_routes_handled(
) -> impl Filter<Extract = (impl warp::Reply,), Error = std::convert::Infallible> + Clone {
    all_routes().recover(error_handler)
}

fn all_routes() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
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

    // Non blocking version of /dummy
    // $ curl 127.0.0.1:8000/dummy_submit
    let dummy_submit = get_dummy_submit();

    // Check if the dummy api is still working
    // $ curl 127.0.0.1:8000/status
    let status = status();

    let health_check = health_check();

    let probe_paths = health_check.or(status);
    let protected_paths = vote.or(show).or(dummy).or(dummy_submit);

    //health_check.or(vote).or(show).or(dummy).or(dummy_submit).or(status)
    probe_paths.or(auth_check().untuple_one().and(protected_paths))
}

// GET /
fn health_check() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::get().and(warp::path::end()).map(|| "ok")
}

/// POST /vote with json body. eg. '{"name": "foo", "votes": 1}'
fn update_votes(
    db: CandidateList,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path!("vote"))
        //.and(auth_check())
        //.untuple_one()
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
        //.and(auth_check())
        //.and(retrieve_token())
        //.and_then(move |token: String| verify_token(token))
        //.untuple_one()
        .and(with_candlist(db))
        .map(|candlist: CandidateList| print_all(candlist))
}

/// GET /dummy
fn get_dummy() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("dummy"))
        .then(dummy_handle_request)
}

/// GET /dummy_submit
fn get_dummy_submit() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    warp::get()
        .and(warp::path!("dummy_submit"))
        .then(dummy_submit_handle_request)
}

/// GET /status
fn status() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::get().and(warp::path!("status")).map(check_status)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handlers::error_handler;
    use crate::models::init_demo_db;

    use std::ops::Deref;

    #[tokio::test]
    async fn test_print_all() {
        let db = init_demo_db();
        let filter = show_all(db).recover(error_handler);
        let resp = warp::test::request().path("/show").reply(&filter).await;
        assert_eq!(resp.status(), 401);
        //assert_eq!(resp.body().len(), 92);
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
