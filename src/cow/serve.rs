
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest, body::BoxBody, http::header::ContentType};
use serde_json::json;
use reqwest::{Client, Url};
use crate::parse_headers::parse_headers;

use super::parse_headers::BareHeaderData;
const VALID_PROTOCOLS: [&str; 4] = ["http:", "https:", "ws:", "wss:"];

pub fn get_header<'a>(req: &'a HttpRequest, name:&str) -> Option<&'a str> {
    req.headers().get(name)?.to_str().ok()
}

async fn fetch(header_data: BareHeaderData) {
    let client:Client = reqwest::Client::new();

    todo!();

    
}



#[get("/v1/")]
pub async fn hello(req: HttpRequest) -> impl Responder {
    let headers = parse_headers(&req);
    if let Result::Err(headers) = headers {
        return headers.respond_to(&req);
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
                "#)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


