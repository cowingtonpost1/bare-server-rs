mod cow;
use std::collections::HashMap;
use std::sync::Arc;

use crate::cow::{bare_errors, parse_headers, serve};

use actix_web::web;
use tokio::sync::Mutex;

// TODO: rename and add fields
struct AppValue;

pub fn configure(cfg: &mut web::ServiceConfig) {
    let mut app_state: Arc<Mutex<HashMap<String, AppValue>>> = Default::default();
    cfg.app_data(web::Data::new(app_state));
    cfg.service(serve::v1_index);
    cfg.service(serve::index);
}
