use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct ModParams {
    n: u32,
    m: u32,
}

fn main() {
    let socket = "127.0.0.1:8080";

    let s = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(show_home))
            .route("/mod", web::post().to(post_mod))
    });
    println!("Starting server at {} ...", socket);
    s.bind(socket)
        .expect("Failed to bind to localhost:8080!")
        .run()
        .expect("Failed to start the server");
}

fn show_home() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
            <title>Modulo Calculator</title>
            <form action="/mod" method="post">
            <input type="text" name="n"/>
            <input type="text" name="m"/>
            <button type="submit">Modulo</button>
            </form>
        "#,
    )
}

fn post_mod(form: web::Form<ModParams>) -> HttpResponse {
    if form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Modulo 0 is not allowed!");
    }

    let resp = format!("{} % {} = <b>{}</b>\n", form.n, form.m, form.n % form.m);
    HttpResponse::Ok().content_type("text/html").body(resp)
}
