use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::string::String;

use regex::Regex;

pub struct MediaFile {
    path: PathBuf,
    base: Arc<PathBuf>
}

impl MediaFile {

    pub fn new(path: &Path, base: &Path) -> MediaFile {
        MediaFile{ path: path.to_owned(), base: Arc::new(base.to_owned()) }
    }

    fn id(&self) -> String {
        // FIXME strip all non ASCII, all colons, and all tab characters
        self.path.strip_prefix(self.base.as_path()).unwrap().to_str().unwrap().to_lowercase()
    }
}

impl Hash for MediaFile {

    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}

pub fn is_media_file(path: &Path) -> bool {
    lazy_static! {
        static ref MEDIA_FILE_EXTENSION: Regex = Regex::new(r"(?i)(mp3)$").unwrap();
    }

    MEDIA_FILE_EXTENSION.is_match(
        // get the extension OsStr, convert to an Option<&str>, and unwrap or return empty string
        path.extension().and_then(|v| v.to_str()).unwrap_or("")
    )
}

#[test]
fn test_is_media_file() {
    // test false cases
    assert!(!is_media_file(Path::new("/home/naftuli/Music/Directory")));
    assert!(!is_media_file(Path::new("/home/naftuli/Music/Directory/Folder.jpg")));

    // test true cases
    assert!(is_media_file(Path::new("/home/naftuli/Music/01 - Track.mp3")));
    assert!(is_media_file(Path::new("/home/naftuli/Music/01 - Track.MP3")));
}

#[test]
fn test_media_file_identity() {
    let base = Path::new("/home/naftuli/Music");

    let f = MediaFile::new(
        Path::new("/home/naftuli/Music/Andrew W. K./I Get Wet/02 - Party Hard.mp3"),
        base
    );

    assert_eq!("andrew w. k./i get wet/02 - party hard.mp3", format!("{}", f.id()));
}
