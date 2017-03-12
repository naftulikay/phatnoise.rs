#[macro_use] extern crate lazy_static;

extern crate crypto;
extern crate id3;
extern crate libc;
#[macro_use]
extern crate log;
extern crate metaflac;
extern crate mp3_duration;
extern crate num_cpus;
extern crate rayon;
extern crate regex;
extern crate unicode_casefold;
extern crate walkdir;

pub mod dms;
pub mod library;
pub mod metadata;
pub mod sync;
pub mod utils;
