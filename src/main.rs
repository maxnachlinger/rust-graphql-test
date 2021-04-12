extern crate serde;
#[macro_use]
extern crate slog;
extern crate config;
use crate::graphql::{build_schema, AppContext};
use crate::logger::{setup_access_logger, setup_logger};
use crate::settings::AppOptions;
use actix_web::{App, HttpServer};
use settings::Settings;

mod graphql;
mod logger;
mod routes;
mod settings;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let logger = setup_logger();

    let settings = Settings::new(&logger).expect("Could not read settings");
    let AppOptions {
        listen_address,
        name: _,
    } = settings.app.clone();

    // graphQL schema
    let schema = build_schema(AppContext {
        logger: logger.clone(),
        settings: settings.clone(),
    });

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .data(settings.clone())
            .wrap(setup_access_logger(&logger))
            .configure(routes::init)
    })
    .bind(listen_address)?
    .system_exit()
    .run()
    .await
}
