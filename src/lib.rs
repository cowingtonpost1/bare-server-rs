mod cow;
use std::collections::HashMap;
use std::sync::Arc;

use crate::cow::{bare_errors, parse_headers, serve};

use actix_web::web;
use cow::serve::AppValue;
use tokio::sync::Mutex;

pub fn configure(cfg: &mut web::ServiceConfig) {
    let app_state: Arc<Mutex<HashMap<String, AppValue>>> = Default::default();
    cfg.app_data(web::Data::new(app_state));
    cfg.service(serve::v1_ws_new_meta);
    cfg.service(serve::v1_ws_meta);
    cfg.service(serve::v1_index);
    cfg.service(serve::index);
}
