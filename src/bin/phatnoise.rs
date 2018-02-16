extern crate log;
extern crate log4rs;
extern crate phatnoise;
extern crate rayon;

use log::LevelFilter;

use log4rs::append::console::ConsoleAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Logger, Root};

use phatnoise::sync::synchronize;

static LOGGING_FORMAT: &'static str = "{d(%Y-%m-%dT%H:%M:%S%.3f%z)} {l:5.5} [{T}] {M}: {m}{n}";

fn configure_logging() {
    let appender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(LOGGING_FORMAT)))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(appender)))
        .logger(Logger::builder().build("id3::frame::stream::v3", LevelFilter::Info))
        .logger(Logger::builder().build("id3::tag", LevelFilter::Info))
        .logger(Logger::builder().build("phatnoise::metadata", LevelFilter::Info))
        .logger(Logger::builder().build("phatnoise::utils::stringpool", LevelFilter::Info))
        .build(Root::builder().appender("stdout").build(LevelFilter::Trace))
        .unwrap();

    log4rs::init_config(config).unwrap();
}

fn configure_rayon() {
    rayon::ThreadPoolBuilder::new().thread_name(|c| format!("rayon-cpu-{:02}", c))
        .build_global().unwrap()
}

fn main() {
    configure_logging();
    configure_rayon();
    synchronize();
}
