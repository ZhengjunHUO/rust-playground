use crate::handlers::{check_status, dummy_handle_request, print_all, update_candidate};
use crate::models::{init_demo_db, Candidate, CandidateList};
use reqwest::Client;
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

    // Check if the dummy api is still working
    // $ 127.0.0.1:8000/status
    let status = status();

    vote.or(show).or(dummy).or(status)
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
        .and(warp::header::optional::<String>("authorization"))
        .and(with_candlist(db))
        .then(|auth: Option<String>, candlist: CandidateList| async {
            match auth {
                None => String::from("No token provided"),
                Some(text) => {
                    //println!("In header: {}", text);
                    if !text.starts_with("Bearer ") {
                        return String::from("Expect a Bearer token");
                    }

                    let endpoint_rslt = std::env::var("AUTH_ENDPOINT");
                    if endpoint_rslt.is_err() {
                        return String::from("Env var AUTH_ENDPOINT not set !");
                    }
                    let (_, token) = text.split_at(7);
                    //println!("Token: {}", token);
                    let client = Client::new();
                    let res = client
                        .get(endpoint_rslt.unwrap())
                        .bearer_auth(token)
                        .send()
                        .await;
                    match res {
                        Ok(resp) => match resp.status().as_u16() {
                            200 => print_all(candlist),
                            401 => {
                                return String::from(
                                    "Failed to authorize, please use a valid token",
                                )
                            }
                            _ => {
                                return format!(
                                    "Unexpected error: [code {}] {}",
                                    resp.status().as_u16(),
                                    resp.text().await.unwrap()
                                )
                            }
                        },
                        Err(e) => return format!("Error occurred: {}", e),
                    }
                }
            }
        })
}

/// GET /dummy
fn get_dummy() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("dummy"))
        .then(|| dummy_handle_request())
}

/// GET /status
fn status() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("status"))
        .map(|| check_status())
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
