use actix_files as fs;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/bare").configure(bare_server_rs::configure))
            .service(
                fs::Files::new("/", "./static/")
                    .prefer_utf8(true)
                    .index_file("index.html"),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
