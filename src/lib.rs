mod cow;
use crate::cow::{bare_errors, parse_headers, serve, websocket_protocol};

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(serve::v1_index);
    cfg.service(serve::index);
}
