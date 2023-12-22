use warp::{
    body::BodyDeserializeError, http::StatusCode, reject::MethodNotAllowed, reply, Rejection, Reply,
};

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
