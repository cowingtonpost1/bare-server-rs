
use crate::parse_headers::ToUri;
use actix_http::{Method, Uri};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest, body::BoxBody, http::header::ContentType, route, FromRequest};
use hyper::{Request, Response};
use serde_json::json;
use crate::parse_headers::parse_headers;

use super::parse_headers::BareHeaderData;
const VALID_PROTOCOLS: [&str; 4] = ["http:", "https:", "ws:", "wss:"];

pub fn get_header<'a>(req: &'a HttpRequest, name:&str) -> Option<&'a str> {
    req.headers().get(name)?.to_str().ok()
}

async fn fetch(header_data: BareHeaderData, a_req: &HttpRequest, mut payload: web::Payload) -> Option<()> {
    let uri = header_data.remote.to_uri().unwrap();

    
    let req = Request::builder().method(a_req.method().as_str()).uri(uri.to_string());
    let mut bytes = web::BytesMut::new();
    while let Some(item) = payload.next().await {
        
        let item = item?;
        bytes.extend_from_slice(&item);
    }
    req.body(bytes);

    let client = hyper::Client::new();

    // POST it...
    let resp = client.request(req).await.ok()?;

    Ok()




    
}



#[route("/v1/", method = "GET", method = "POST", method ="PUT", method = "DELETE", method = "OPTIONS", method = "HEAD", method = "CONNECT", method = "PATCH", method = "TRACE")]
pub async fn hello(req: HttpRequest) -> impl Responder {
    let headers = parse_headers(&req);
    if let Result::Err(headers) = headers {
        return headers.respond_to(&req);
    } else {
    if let Some(payload) = web::Payload::extract(&req).await.ok() {
        fetch(headers.unwrap(), &req, payload);
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
                "#)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


