use actix_web::{
    body::BoxBody, get, http::header::ContentType, post, web, App, HttpRequest, HttpResponse,
    HttpServer, Responder,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(bare_server_rs::configure))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
