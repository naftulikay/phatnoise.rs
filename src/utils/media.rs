use std::path::Path;
use std::path::PathBuf;
use regex::Regex;

use walkdir::WalkDir;

lazy_static! {
    static ref MEDIA_FILE_EXTENSION: Regex = Regex::new(r"(?i)(mp3)$").unwrap();
}

pub fn get_media_library(base: &Path) -> Vec<PathBuf> {
    let walker = WalkDir::new(base).min_depth(1).into_iter();
    // find all media files and collect them
    walker.filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| is_media_filename(e.path()))
        .map(|e| e.path().to_owned())
        .collect()
}

pub fn is_media_filename(path: &Path) -> bool {
    MEDIA_FILE_EXTENSION.is_match(
        // get the extension OsStr, convert to an Option<&str>, and unwrap or return empty string
        path.extension().and_then(|v| v.to_str()).unwrap_or("")
    )
}

#[test]
fn test_is_media_filename() {
    // test false cases
    assert!(!is_media_filename(Path::new("/home/naftuli/Music/Directory")));
    assert!(!is_media_filename(Path::new("/home/naftuli/Music/Directory/Folder.jpg")));

    // test true cases
    assert!(is_media_filename(Path::new("/home/naftuli/Music/01 - Track.mp3")));
    assert!(is_media_filename(Path::new("/home/naftuli/Music/01 - Track.MP3")));
}
