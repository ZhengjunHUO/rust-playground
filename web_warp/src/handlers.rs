//use anyhow::{bail, Result};
use crate::models::{CandidateList, IS_WORKING};
use reqwest::Client;
use warp::{
    body::BodyDeserializeError, http::StatusCode, reject, reject::MethodNotAllowed, reply, Filter,
    Rejection, Reply,
};

#[derive(Debug)]
struct InvalidToken;

impl warp::reject::Reject for InvalidToken {}

#[derive(Debug)]
struct MissingEnvVar;

impl warp::reject::Reject for MissingEnvVar {}

pub(crate) fn with_candlist(
    list: CandidateList,
) -> impl Filter<Extract = (CandidateList,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || list.clone())
}

pub(crate) fn auth_check() -> impl Filter<Extract = ((),), Error = Rejection> + Copy {
    retrieve_token().and_then(move |token: String| verify_token(token))
}

fn retrieve_token() -> impl Filter<Extract = (String,), Error = Rejection> + Copy {
    warp::header::optional::<String>("authorization").and_then(|auth: Option<String>| async move {
        match auth {
            None => {
                println!("[DEBUG] No token is provided !");
                return Err(reject::custom(InvalidToken));
            }
            Some(text) => {
                //println!("In header: {}", text);
                if !text.starts_with("Bearer ") {
                    println!("[DEBUG] Expect a Bearer token");
                    return Err(reject::custom(InvalidToken));
                }

                let (_, token) = text.split_at(7);
                println!("[DEBUG] Recv token: {}", token);
                return Ok(token.to_owned());
            }
        }
    })
}

async fn verify_token(token: String) -> Result<(), Rejection> {
    let endpoint_rslt = std::env::var("AUTH_ENDPOINT");
    if endpoint_rslt.is_err() {
        println!("[DEBUG] Env var AUTH_ENDPOINT not set !");
        return Err(reject::custom(MissingEnvVar));
    }

    let client = Client::new();
    let res = client
        .get(endpoint_rslt.unwrap())
        .bearer_auth(token.clone())
        .send()
        .await;
    match res {
        Ok(resp) => match resp.status().as_u16() {
            200 => return Ok(()),
            401 => {
                println!("[DEBUG] Failed to authorize, please use a valid token",);
                return Err(reject::custom(InvalidToken));
            }
            _ => {
                println!(
                    "[DEBUG] Unexpected error: [code {}] {}",
                    resp.status().as_u16(),
                    resp.text().await.unwrap()
                );
                return Err(reject::custom(InvalidToken));
            }
        },
        Err(e) => {
            println!("[DEBUG] Error occurred: {}", e);
            return Err(reject::custom(InvalidToken));
        }
    }
}

pub(crate) fn update_candidate(name: &str, votes: u32, cands: CandidateList) -> String {
    let mut guard = cands.lock().unwrap();
    guard
        .entry(name.to_string())
        .and_modify(|sum| *sum += votes)
        .or_insert(votes);
    println!("[DEBUG] {:?}", guard);
    format!("{} got {} vote(s) !\n", name, *guard.get(name).unwrap())
}

pub(crate) fn print_all(cands: CandidateList) -> String {
    let guard = cands.lock().unwrap();
    guard
        .iter()
        .map(|(name, sum)| format!("{} currently has {} vote(s) !\n", name, sum))
        .collect::<String>()
}

pub(crate) async fn dummy_handle_request() -> Result<impl warp::Reply, std::convert::Infallible> {
    use std::{thread, time};

    {
        if *IS_WORKING.lock().unwrap() {
            return Ok(reply::with_status(
                "Still working, try later !",
                StatusCode::TOO_MANY_REQUESTS,
            ));
        }
    }

    {
        *IS_WORKING.lock().unwrap() = true;
    }

    // Do something heavy
    thread::sleep(time::Duration::from_secs(10));

    {
        *IS_WORKING.lock().unwrap() = false;
    }

    Ok(reply::with_status("Done", StatusCode::OK))
}

pub(crate) fn check_status() -> Result<impl warp::Reply, std::convert::Infallible> {
    if *IS_WORKING.lock().unwrap() {
        Ok(reply::with_status(
            "Still working, try later !",
            StatusCode::TOO_MANY_REQUESTS,
        ))
    } else {
        Ok(reply::with_status("Idle", StatusCode::OK))
    }
}

pub async fn error_handler(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    if err.is_not_found() {
        Ok(reply::with_status(
            "Invalid request !",
            StatusCode::NOT_FOUND,
        ))
    } else if let Some(_e) = err.find::<BodyDeserializeError>() {
        Ok(reply::with_status(
            "Error(s) in payload !",
            StatusCode::BAD_REQUEST,
        ))
    } else if let Some(_e) = err.find::<MissingEnvVar>() {
        Ok(reply::with_status(
            "The api server is running without connection to the SSO server!",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(_e) = err.find::<InvalidToken>() {
        Ok(reply::with_status(
            "Please provide a valid token !",
            StatusCode::UNAUTHORIZED,
        ))
    } else if let Some(_e) = err.find::<MethodNotAllowed>() {
        Ok(reply::with_status(
            "Illegal request !",
            StatusCode::METHOD_NOT_ALLOWED,
        ))
    } else {
        println!("[DEBUG] Unhandled rejection: {:?}", err);
        Ok(reply::with_status(
            "INTERNAL_SERVER_ERROR",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}
