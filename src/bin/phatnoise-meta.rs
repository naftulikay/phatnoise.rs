extern crate phatnoise;

#[macro_use]
extern crate log;
extern crate log4rs;
extern crate num_cpus;
extern crate rayon;

use log::LevelFilter;

use log4rs::append::console::ConsoleAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Logger, Root};

use phatnoise::library::get_local_media_library;
use phatnoise::metadata::MediaMetadata;
use phatnoise::utils::StringPool;

use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

use std::path::Path;

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
    rayon::ThreadPoolBuilder::new().thread_name(|c| format!("rayon-{:02}", c))
        .build_global().unwrap()
}

fn main() {
    configure_rayon();
    configure_logging();

    info!("Scanning media library...");

    let library_dir = Path::new("/home/naftuli/Music");
    let pool = StringPool::new();
    // convert local media library into a sequence of PathBufs, then into a sequence of
    // Result<MediaMetadata>, then bounce down to MediaMetadata
    let threadpool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_cpus::get() * 2)
        .thread_name(|c| format!("io-{:02}", c))
        .build()
        .unwrap();

    threadpool.install(|| {
        let mut files = get_local_media_library(&library_dir).into_par_iter()
            .map(|l| MediaMetadata::load(&l.path, &library_dir, &pool))
            .filter_map(|m| m.ok())
            .collect::<Vec<MediaMetadata>>();
        files.sort_by(MediaMetadata::by_artist);
    });
}
