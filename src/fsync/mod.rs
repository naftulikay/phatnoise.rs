use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::string::String;

use regex::Regex;
use unicode_casefold::UnicodeCaseFold;

pub struct MediaFile {
    path: PathBuf,
    base: Arc<PathBuf>
}

static SPACE_CHARS: &'static [char] = &['\t'];
static DELETE_CHARS: &'static [char] = &[':'];

impl MediaFile {

    pub fn new<P1, P2>(path: P1, base: P2) -> MediaFile
            where P1: Into<PathBuf>, P2: Into<PathBuf> {
        MediaFile{ path: path.into(), base: Arc::new(base.into()) }
    }

    fn id(&self) -> String {
        self.path.strip_prefix(self.base.as_path()).unwrap().to_string_lossy().chars()
            //  delete illegal characters
            .filter(|c| !DELETE_CHARS.contains(&c))
            // replace certain characters with a single space character
            .map(|c| if SPACE_CHARS.contains(&c) { ' ' } else { c })
            //  bounce down to lowercase
            .flat_map(|c| c.case_fold())
            //  collect into a string
            .collect()
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
    let base = Path::new("Music");

    // test bounce to lowercase
    assert_eq!(
        "andrew w. k./i get wet/02 - party hard.mp3",
        MediaFile::new("Music/Andrew W. K./I Get Wet/02 - Party Hard.mp3", base).id()
    );
    // test supported ascii characters
    assert_eq!(
        "mêlée/everyday behavior/01 - got it all.mp3",
        MediaFile::new("Music/Mêlée/Everyday Behavior/01 - Got It All.mp3", base).id()
    );
    // test strip colons
    assert_eq!(
        "apocalyptica/begin again/01 - track thing.mp3",
        MediaFile::new("Music/Apocalyptica/Begin: Again/01 - Track: Thing.mp3", base).id()
    );
    // test strip tabs
    assert_eq!(
        "the end/something.mp3",
        MediaFile::new("Music/The\tEnd/Something.mp3", base).id()
    );
}
