use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest, body::BoxBody, http::header::ContentType};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().configure(bare_server_cow::configure)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


