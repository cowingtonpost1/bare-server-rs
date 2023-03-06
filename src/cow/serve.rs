use std::collections::HashMap;
use std::str::FromStr;

use crate::parse_headers::parse_headers;
use crate::parse_headers::ApplyHeaders;
use crate::parse_headers::ToUri;
use actix_web::{
    get, http::header::ContentType, route, web, FromRequest, HttpRequest, HttpResponse, Responder,
};
use http::HeaderValue;

use super::bare_errors::BareError;

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

pub async fn v1_index(req: HttpRequest, mut payload: web::Payload) -> impl Responder {
    let headers = parse_headers(&req);
    if let Result::Err(headers) = headers {
        return headers.respond_to(&req);
    } else {
        if let Some(_payload) = web::Payload::extract(&req).await.ok() {
            let headers = headers.unwrap();
            let _client = awc::Client::default();

            let _req_uri = headers.remote.to_uri().unwrap();

            println!("{:#?}", _req_uri);

            let mut c_req = _client.request(req.method().clone(), _req_uri);
            headers.headers.apply_headers(&mut c_req);

            let re = c_req.send_stream(payload).await;

            if let Err(_err) = re {
                // TODO: make errors compiliant with specification
                println!("{:#?}", _err);
                return BareError {
                    code: "HOST_NOT_FOUND".to_owned(),
                    id: "a".to_owned(),
                    message: "cow".to_owned(),
                }
                .respond_to(&req);
            }

            let re = re.unwrap();

            let mut response = HttpResponse::Ok();

            response.insert_header((
                "X-Bare-Status",
                HeaderValue::from_str(re.status().as_str()).unwrap(),
            ));
            response.insert_header((
                "X-Bare-Status-text",
                HeaderValue::from_str(format!("{:?}", re.status()).as_str()).unwrap(),
            ));

            let mut new_headers: HashMap<String, String> = HashMap::new();

            re.headers().into_iter().for_each(|(k, v)| {
                new_headers.insert(k.to_string(), v.to_str().unwrap().to_owned());
            });

            response.insert_header((
                "X-Bare-Headers",
                HeaderValue::from_str(serde_json::to_string(&new_headers).unwrap().as_str())
                    .unwrap(),
            ));

            return response.streaming(re);
        }
    }
    HttpResponse::Ok().body("Hello world!")
}

#[get("/")]
pub async fn index(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().content_type(ContentType::json()).body(
        r#"
        {
            "versions": [
                "v1"
            ],
            "language": "Rust",
            "maintainer": {
                "email": "cowingtonpost@gmail.com",
                "website": "https://github.com/cowingtonpost1/bare-server-rs/"
	        },
            "project": {
                "name": "bare-server-rs",
                "description": "A TompHTTP Bare Server V1 Implementation in Rust",
                "email": "cowingtonpost@gmail.com",
                "website": "https://github.com/cowingtonpost1/bare-server-rs/",
                "repository": "https://github.com/cowingtonpost1/bare-server-rs/",
                "version": "0.0.0"
            }
        }
                "#,
    )
}
