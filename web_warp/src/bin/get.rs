use warp::{http::StatusCode, path, reject, reply, Filter, Rejection, Reply};

#[derive(Debug)]
struct IllegalInput;

impl reject::Reject for IllegalInput {}

#[tokio::main]
async fn main() {
    // curl 127.0.0.1:8000
    let root = path::end().map(|| "The journey begins here!\nCall /huo for help!\n");

    let help = path("huo")
        .and(path::end())
        .map(|| "You can call /huo/<YOUR_NAME>/<YOUR_LOCATION>!\n");

    // curl 127.0.0.1:8000/huo/fufu/paris
    let huo = warp::get()
        .and(check_params())
        .and(warp::header("user-agent"))
        .map(|c: (String, String), agent: String| {
            format!("huo: Welcome {} from {} using {}!\n", c.0, c.1, agent)
        })
        .recover(handler);

    let routes = warp::get().and(root.or(help)).or(huo);
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn check_params() -> impl Filter<Extract = ((String, String),), Error = Rejection> + Copy {
    path!("huo" / String / String).and_then(|name: String, place| async move {
        if name.as_str() == "foo" || name.as_str() == "bar" {
            return Err(reject::custom(IllegalInput));
        }

        Ok((name, place))
    })
}

async fn handler(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    if err.is_not_found() {
        Ok(reply::with_status(
            "Invalid request !",
            StatusCode::NOT_FOUND,
        ))
    } else if let Some(_e) = err.find::<IllegalInput>() {
        Ok(reply::with_status("FORBIDDEN", StatusCode::FORBIDDEN))
    } else {
        println!("[DEBUG] Unhandled rejection: {:?}", err);
        Ok(reply::with_status(
            "INTERNAL_SERVER_ERROR",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}
