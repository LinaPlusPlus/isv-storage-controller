use tracing::{span, Level, instrument};
use tracing_appender::{non_blocking::WorkerGuard, rolling::RollingFileAppender};
use std::os::unix::net::SocketAddr;
use serde::Deserialize;
use tracing_subscriber::{fmt::format::FmtSpan, util::SubscriberInitExt};

#[derive(Deserialize,Debug)]
#[serde(default)]
pub struct Config {
    pub address: String,
    pub logdir: String
}

impl Default for Config {
    fn default() -> Self {
        Config {
            address: String::from("127.0.0.1:2369"),
            logdir: String::from("./log/"),
        }
    }
}

#[derive(Debug)]
pub struct Backbone {
    pub config: Config,
}

#[instrument]
pub fn setup_backbone() -> (Backbone, WorkerGuard) {

    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("Problem in your config: {:#?}", error), //TODO better error handling here
    };

    let file_appender = tracing_appender::rolling::daily(config.logdir.clone(), "isv-api");

    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    // Set up the subscriber to write logs to the file
    tracing_subscriber::fmt()
    .with_writer(non_blocking)
    .with_max_level(Level::DEBUG)
    .with_ansi(false)
    .finish()
    .init();

    (Backbone { config },guard)
}