# Bare Server Written in Rust

An implementation of the [TompHTTP Bare Server V1 Specification](https://github.com/tomphttp/specifications/blob/master/BareServerV1.md) in Rust.

This is highly work in progress and does not completely work. Simple HTTP
requests somewhat work. A basic Ultraviolet frontend works with simple HTTP and
some HTTPS sites. More debugging still needs to be done and work needs to be
started on the Web Socket API. Specifically, Google and duck duck go do not work, startpage does work.

## Programmatically creating a bare server in Actix
```rust
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
```

## Using the CLI
```sh
bare-server-rs
```
This will start a bare server on port 8080 serving static files from ./static/
and the bare server on ./bare.

## TODO
- Fix/Debug proxied requests for some sites.
- Implement the Web Socket routes.


