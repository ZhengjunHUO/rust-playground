use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct ModParams {
    n: u32,
    m: u32,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let s = HttpServer::new(|| {
        App::new()
            .service(show_home)
            .service(post_mod)
    });
    println!("Starting server at 8080");
    s.bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[get("/")]
async fn show_home() -> HttpResponse {
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

#[post("/mod")]
async fn post_mod(form: web::Form<ModParams>) -> HttpResponse {
    if form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Modulo 0 is not allowed!");
    }

    let resp = format!("{} % {} = <b>{}</b>\n", form.n, form.m, form.n % form.m);
    HttpResponse::Ok().content_type("text/html").body(resp)
}
