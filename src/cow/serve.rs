use std::collections::HashMap;
use std::sync::Arc;

use actix::prelude::*;

use actix_web_actors::ws;
use rand::prelude::*;
use serde::Serialize;
use tokio::sync::Mutex;

use serde::Deserialize;

use crate::parse_headers::parse_headers;
use crate::parse_headers::ApplyHeaders;
use crate::parse_headers::ToUri;
use actix_web::{
    get, http::header::ContentType, route, web, FromRequest, HttpRequest, HttpResponse, Responder,
};
use http::HeaderValue;

use super::bare_errors::BareError;
use super::parse_headers::BareRemote;
use super::websocket_protocol::*;

const VALID_PROTOCOLS: [&str; 4] = ["http:", "https:", "ws:", "wss:"];

#[derive(Serialize, Deserialize, Debug)]
pub struct AppValue {
    remote: Option<BareRemote>,
    headers: Option<HashMap<String, String>>,
    forward_headers: Option<Vec<String>>,
    id: Option<String>,
}

impl Default for AppValue {
    fn default() -> Self {
        return AppValue {
            remote: None,
            headers: None,
            forward_headers: None,
            id: None,
        };
    }
}

struct BareWebsocket {
    mystate: AppValue,
}

impl Actor for BareWebsocket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for BareWebsocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

pub fn get_header<'a>(req: &'a HttpRequest, name: &str) -> Option<&'a str> {
    req.headers().get(name)?.to_str().ok()
}

fn random_hex(byte_length: usize) -> String {
    let mut bytes = vec![0u8; byte_length];
    let mut rng = rand::thread_rng();
    rng.fill_bytes(&mut bytes);
    bytes
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>()
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

pub async fn v1_index(req: HttpRequest, payload: web::Payload) -> HttpResponse {
    if let Some(upgrade) = get_header(&req, "Upgrade") {
        if upgrade.eq("websocket") {
            if let Some(ws_protocol_header) = get_header(&req, "Sec-WebSocket-Protocol") {
                let json = {
                    let a: Vec<&str> = ws_protocol_header.split(",").collect();
                    let b = a.get(1).unwrap();
                    if is_valid_protocol(b) {
                        decode_protocol(b).unwrap()
                    } else {
                        "".to_string()
                    }
                };

                let json: AppValue = serde_json::from_str(json.as_str()).unwrap();
                let re = ws::start(BareWebsocket { mystate: json }, &req, payload);
                if let Ok(r) = re {
                    return r;
                }
                return re.unwrap_err().into();
            } else {
                return BareError {
                    code: "A".to_string(),
                    id: "a".to_string(),
                    message: "a".to_string(),
                }
                .respond_to(&req);
            }
        }
    }
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

            println!("{:#?}", c_req);

            let re = c_req.send_stream(payload).await;

            if let Err(_err) = re {
                // TODO: make errors compiliant with specification
                println!("E: {:#?}", _err);
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

#[get("/v1/ws-new-meta")]
pub async fn v1_ws_new_meta(
    _req: HttpRequest,
    app_state: web::Data<Arc<Mutex<HashMap<String, AppValue>>>>,
) -> impl Responder {
    let new_hex = random_hex(8);
    let mut new_value: AppValue = Default::default();
    new_value.id = Some(new_hex.clone());
    app_state.lock().await.insert(new_hex.clone(), new_value);
    return HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body(new_hex);
}

#[get("/v1/ws-meta")]
pub async fn v1_ws_meta(
    req: HttpRequest,
    app_state: web::Data<Arc<Mutex<HashMap<String, AppValue>>>>,
) -> impl Responder {
    if let Some(bare_id) = get_header(&req, "X-Bare-ID") {
        let state = app_state.lock().await;
        println!("{:#?}", state);
        let value = state.get(bare_id);

        let body = serde_json::to_string(&value).unwrap();
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body);
    }
    return BareError {
        code: "MISSING_BARE_HEADER".to_string(),
        id: "X-Bare-ID".to_string(),
        message: "Missing header X-Bare-ID".to_string(),
    }
    .respond_to(&req);
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
