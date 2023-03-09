A function that can be passed to the actix_web configure route. 

## Example
```rust
HttpServer::new(|| {
    App::new()
        .service(web::scope("/bare").configure(bare_server_rs::configure))
})
```
