use warp::{path, Filter};

#[tokio::main]
async fn main() {
    // curl 127.0.0.1:8000
    let root = path::end().map(|| "The journey begins here!\nCall /huo for help!\n");

    let help = path("huo")
        .and(path::end())
        .map(|| "You can call /huo/<YOUR_NAME>/<YOUR_LOCATION>!\n");

    // curl 127.0.0.1:8000/huo/fufu/paris
    let huo = path!("huo" / String / String)
        .and(warp::header("user-agent"))
        .map(|name, place, agent: String| {
            format!("huo: Welcome {} from {} using {}!\n", name, place, agent)
        });

    let routes = warp::get().and(root.or(help).or(huo));
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
