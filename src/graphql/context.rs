use crate::settings::Settings;
use actix_web::http::HeaderMap;
use slog::Logger;

#[derive(Clone)]
pub struct AppContext {
    pub settings: Settings,
    pub logger: Logger,
}

#[derive(Clone)]
pub struct RequestContext {
    pub request_id: String,
    pub headers: HeaderMap,
}
