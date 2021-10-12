use crate::settings::Settings;
use actix_web::http::HeaderMap;
use slog::Logger;
use std::collections::HashMap;

#[derive(Clone)]
pub struct AppContext {
    pub settings: Settings,
    pub logger: Logger,
}

#[derive(Clone)]
pub struct RequestContext {
    pub traceparent: String,
    pub headers: HeaderMap,
    pub cookies_hash_map: HashMap<String, String>,
}
