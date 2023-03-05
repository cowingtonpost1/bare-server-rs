use crate::parse_headers::parse_headers;
use crate::parse_headers::ToUri;
use actix_web::{
    body::BoxBody, get, http::header::ContentType, post, route, web, App, FromRequest, HttpRequest,
    HttpResponse, HttpServer, Responder,
};
use serde_json::json;

use super::parse_headers::BareHeaderData;
const VALID_PROTOCOLS: [&str; 4] = ["http:", "https:", "ws:", "wss:"];

pub fn get_header<'a>(req: &'a HttpRequest, name: &str) -> Option<&'a str> {
    req.headers().get(name)?.to_str().ok()
}

#[route(
    "/v1/",
    method = "GET",
    method = "POST",
    method = "PUT",
    method = "DELETE",
    method = "OPTIONS",
    method = "HEAD",
    method = "CONNECT",
    method = "PATCH",
    method = "TRACE"
)]
pub async fn hello(req: HttpRequest) -> impl Responder {
    let headers = parse_headers(&req);
    if let Result::Err(headers) = headers {
        return headers.respond_to(&req);
    } else {
        if let Some(payload) = web::Payload::extract(&req).await.ok() {
            let mut client = awc::Client::default();

            let req_uri = headers.unwrap().remote.to_uri().unwrap();
        }
    }
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/")]
pub async fn index(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().content_type(ContentType::json()).body(
        r#"
        {
            "versions": [
                "v1"
            ],
            "language": "Rust"
            }
                "#,
    )
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
