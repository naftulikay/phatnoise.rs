use regex::Regex;

use std::cmp::{Eq, PartialEq};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::string::String;
use std::sync::Arc;

use unicode_casefold::UnicodeCaseFold;

use walkdir::{DirEntry, WalkDir};

pub struct MediaFile {
    pub path: PathBuf,
    base: Arc<PathBuf>,
}

static SPACE_CHARS: &'static [char] = &['\t'];
static DELETE_CHARS: &'static [char] = &[':'];

impl MediaFile {
    pub fn new<P1, P2>(path: P1, base: P2) -> MediaFile
    where
        P1: Into<PathBuf>,
        P2: Into<PathBuf>,
    {
        MediaFile {
            path: path.into(),
            base: Arc::new(base.into()),
        }
    }

    pub fn id(&self) -> String {
        self.path
            .strip_prefix(self.base.as_path())
            .unwrap()
            .to_string_lossy()
            .chars()
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

impl Eq for MediaFile {}

impl Hash for MediaFile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}

impl PartialEq for MediaFile {
    fn eq(&self, other: &MediaFile) -> bool {
        self.id() == other.id()
    }
}

pub fn is_media_file(path: &Path) -> bool {
    lazy_static! {
        static ref MEDIA_FILE_EXTENSION: Regex = Regex::new(r"(?i)(mp3)$").unwrap();
    }

    MEDIA_FILE_EXTENSION.is_match(
        // get the extension OsStr, convert to an Option<&str>, and unwrap or return empty string
        path.extension().and_then(|v| v.to_str()).unwrap_or(""),
    )
}

/// Fetch a list of all media files in the local media library.
pub fn get_local_media_library<P>(base: P) -> HashSet<MediaFile>
where
    P: Into<PathBuf>,
{
    // mask base with a reference counter
    let base: Arc<PathBuf> = Arc::new(base.into());

    // find all files and directories within the directory excluding the directory itself
    let walker = WalkDir::new(base.clone().as_path())
        .min_depth(1)
        .into_iter();

    // find all media files and collect them as MediaFile instances
    walker
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && is_media_file(e.path()))
        .map(|e| MediaFile::new(e.path(), base.clone().as_path()))
        .collect()
}

/// Fetch a list of all media files in the DMS media library
pub fn get_dms_media_library<P>(base: P) -> HashSet<MediaFile>
where
    P: Into<PathBuf>,
{
    // mask base with a reference counter
    let base: Arc<PathBuf> = Arc::new(base.into());

    // find all files and directories within the directory excluding the directory itself
    let walker = WalkDir::new(base.clone().as_path())
        .min_depth(1)
        .into_iter();

    // find all media files not existing in profiles and tts due to these being data directories
    walker
        .filter_entry(|e| is_allowed_dms_path(base.clone().as_path(), e))
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && is_media_file(e.path()))
        .map(|e| MediaFile::new(e.path(), base.clone().as_path()))
        .collect()
}

pub fn is_allowed_dms_path(base: &Path, entry: &DirEntry) -> bool {
    // get the lowercase intersection of the entry and the base
    let intersection = entry
        .path()
        .strip_prefix(base)
        .unwrap()
        .to_string_lossy()
        .chars()
        .flat_map(|c| c.case_fold())
        .collect::<String>();

    match intersection.as_str() {
        "profiles" => false,
        "tts" => false,
        _ => true,
    }
}

#[test]
fn test_is_media_file() {
    // test false cases
    assert!(!is_media_file(Path::new("/home/naftuli/Music/Directory")));
    assert!(!is_media_file(Path::new(
        "/home/naftuli/Music/Directory/Folder.jpg"
    )));

    // test true cases
    assert!(is_media_file(Path::new(
        "/home/naftuli/Music/01 - Track.mp3"
    )));
    assert!(is_media_file(Path::new(
        "/home/naftuli/Music/01 - Track.MP3"
    )));
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
        MediaFile::new(
            "Music/Apocalyptica/Begin: Again/01 - Track: Thing.mp3",
            base
        )
        .id()
    );
    // test strip tabs
    assert_eq!(
        "the end/something.mp3",
        MediaFile::new("Music/The\tEnd/Something.mp3", base).id()
    );
}
