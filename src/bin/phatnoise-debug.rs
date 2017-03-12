extern crate phatnoise;

#[macro_use]
extern crate log;
extern crate log4rs;
extern crate rayon;

use log::LevelFilter;

use log4rs::append::console::ConsoleAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

use phatnoise::dms;
use phatnoise::library;
use phatnoise::sync;

use std::path::PathBuf;

static LOGGING_FORMAT: &'static str = "{d(%Y-%m-%d %H:%M:%S)} {l:5.5} [{T}] {M}: {m}{n}";

fn configure_rayon() {
    rayon::ThreadPoolBuilder::new().thread_name(|c| format!("rayon-cpu-{:02}", c))
        .build_global().unwrap()
}

fn configure_logs() {
    let appender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(LOGGING_FORMAT)))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(appender)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Debug))
        .unwrap();

    log4rs::init_config(config).unwrap();
}

fn main () {
    configure_logs();
    configure_rayon();
    debug_dms();
    debug_library();
}

fn debug_library() {
    let local_library = library::get_local_media_library(&PathBuf::from("/home/naftuli/Music"));
    let dms_library = library::get_dms_media_library();

    let added_files = sync::added_files(&local_library, &dms_library);
    let deleted_files = sync::deleted_files(&local_library, &dms_library);
    let changed_files = sync::changed_files(&local_library, &dms_library);

    info!("Library: Local Files: {}; DMS Files: {}", local_library.len(), dms_library.len());
    info!("Files on Local But Not DMS: {}", added_files.len());
    info!("Files on DMS But Not Local: {}", deleted_files.len());
    info!("Updated Files: {}", changed_files.len());
}

fn debug_dms() {
    let dms_present = dms::is_dms_present();
    let dms_mounted = dms::is_dms_mounted();

    let dms_device = match dms::get_dms_device() {
        Some(pathbuf) => pathbuf.to_string_lossy().into_owned(),
        None => "(null)".to_owned(),
    };

    let dms_mountpoint = match dms::get_dms_mount_point() {
        Some(pathbuf) => String::from(pathbuf.to_string_lossy()),
        None => String::from("(null)")
    };

    info!("DMS: is present? {}; device: {}; is mounted? {}; mountpoint: {}",
        dms_present, dms_device, dms_mounted, dms_mountpoint,
    );
}
