use actix_slog::StructuredLogger;
use chrono::{DateTime, SecondsFormat, Utc};
use slog::{Drain, FnValue, Logger, PushFnValue};
use slog_json::Json;
use std::time::SystemTime;

pub fn setup_logger() -> Logger {
    let json_log_drain_sync = Json::new(std::io::stdout())
        .add_key_value(o!(
        "ts" => PushFnValue(move |_, ser| {
            // we want to log in UTC
            let now: DateTime<Utc> = SystemTime::now().into();
            ser.emit(now.to_rfc3339_opts(SecondsFormat::Secs, true))
        }),
        "level" => FnValue(move |rinfo| {
            rinfo.level().as_str()
        }),
        "msg" => PushFnValue(move |record, ser| {
            ser.emit(record.msg())
        }),
        ))
        .build()
        .fuse();

    let log_drain = slog_async::Async::new(json_log_drain_sync).build().fuse();

    Logger::root(
        log_drain,
        o!("version" => env!("CARGO_PKG_VERSION"), "service" => env!("CARGO_PKG_NAME"), "log_type" => "application", "application_type" => "service", "module" => FnValue(move |info| {
            info.module().to_string()
        })
        ),
    )
}

pub fn setup_access_logger(logger: &Logger) -> StructuredLogger {
    StructuredLogger::new(logger.new(
        o!("log_type" => "access", "protocol" => "http", "server_name" => env!("CARGO_PKG_NAME")),
    ))
    // we don't need to log hits to these routes
    .exclude("/favicon.ico")
    .exclude("/ecv")
}
