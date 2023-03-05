mod cow;
use crate::cow::{bare_errors, parse_headers, serve, websocket_protocol};

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(serve::echo);
    cfg.service(serve::hello);
    cfg.service(serve::index);
}
