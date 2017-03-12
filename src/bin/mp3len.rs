extern crate log;
extern crate log4rs;
extern crate rayon;
extern crate simplemad;
extern crate mp3_duration;

use log::LevelFilter;

use log4rs::append::console::ConsoleAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Logger, Root};

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

static LOGGING_FORMAT: &'static str = "{d(%Y-%m-%dT%H:%M:%S%.3f%z)} {l:5.5} [{T}] {M}: {m}{n}";

const FIXED: bool = false;

fn configure_logging() {
    let appender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(LOGGING_FORMAT)))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(appender)))
        .logger(Logger::builder().build("id3::frame::stream::v3", LevelFilter::Info))
        .logger(Logger::builder().build("id3::tag", LevelFilter::Info))
        .logger(Logger::builder().build("phatnoise::metadata", LevelFilter::Info))
        .build(Root::builder().appender("stdout").build(LevelFilter::Trace))
        .unwrap();

    log4rs::init_config(config).unwrap();
}

fn configure_rayon() {
    rayon::ThreadPoolBuilder::new().thread_name(|c| format!("rayon-cpu-{:02}", c))
        .build_global().unwrap()
}

fn main() {
    configure_rayon();
    configure_logging();

    let p = if FIXED {
        Path::new("/home/naftuli/Music/Daft Punk/Discovery/01 - One More Time.mp3")
    } else {
        Path::new("/home/naftuli/Music/Daft Punk/TRON Legacy [Special Edition]/01 - Overture.mp3")
    };

    let f = File::open(p).unwrap();
    let mut decoder = simplemad::Decoder::decode(BufReader::new(f)).unwrap();

    loop {
        let frame = match decoder.get_frame() {
            Err(simplemad::SimplemadError::EOF) => {
                break;
            },
            result => result
        };

        if !frame.is_ok() {
            continue;
        }

        let frame = frame.unwrap();
        println!("end: {:?}", (frame.position + frame.duration).as_secs());
    }

    println!("{:?}", mp3_duration::from_path(p));
}
