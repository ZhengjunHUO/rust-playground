use crate::models::{CandidateList, IS_WORKING};
use warp::{
    body::BodyDeserializeError, http::StatusCode, reject::MethodNotAllowed, reply, Rejection, Reply,
};

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
