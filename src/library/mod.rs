use dms;

use std::cmp::Eq;
use std::cmp::PartialEq;
use std::collections::HashSet;
use std::hash::Hash;
use std::hash::Hasher;
use std::path::Path;
use std::path::PathBuf;

use utils;

use unicode_casefold::UnicodeCaseFold;

#[derive(Clone,Copy,Eq,Debug,PartialEq)]
pub enum LibrarySource {
    Local,
    DMS
}

#[derive(Debug)]
pub struct LibraryFile {
    pub id: String,
    pub path: PathBuf,
    pub base: PathBuf,
    pub source: LibrarySource
}

impl LibraryFile {

    pub fn new(path: &Path, base: &Path, source: LibrarySource) -> Self {
        LibraryFile {
            id: LibraryFile::gen_id(&path, &base),
            path: path.to_path_buf(),
            base: base.to_path_buf(),
            source: source
        }
    }

    /// Generate an ID for the media file at the given path with the given base path.
    ///
    /// This is used for path uniqueness checks between two media libraries, ie between the local
    /// media library on disk and the remote media library on the DMS.
    fn gen_id(path: &Path, base: &Path) -> String {
        path.strip_prefix(base).unwrap().to_string_lossy().chars()
            //  delete illegal characters
            .filter(|c| !utils::FAT32_DELETE_CHARS.contains(&c))
            // replace certain characters with a single space character
            .map(|c| if utils::FAT32_HYPHENIZE_CHARS.contains(&c) { '-' } else { c })
            //  bounce down to lowercase
            .flat_map(|c| c.case_fold())
            //  collect into a string
            .collect()
    }

    pub fn debase(&self) -> &str {
        self.path.strip_prefix(&self.base).unwrap().to_str().unwrap()
    }
}

impl Eq for LibraryFile {}

impl PartialEq for LibraryFile {
    fn eq(&self, other: &LibraryFile) -> bool {
        self.id == other.id
    }
}

impl Hash for LibraryFile {

    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

pub fn get_local_media_library(base: &Path) -> HashSet<LibraryFile> {
    utils::media::get_media_library(base).iter().map(|p| {
        LibraryFile::new(p, base, LibrarySource::Local)
    }).collect()
}

pub fn get_dms_media_library() -> HashSet<LibraryFile> {
    match dms::get_dms_mount_point() {
        Some(base) => {
            utils::media::get_media_library(&base).iter().map(|p| {
                LibraryFile::new(p, &base, LibrarySource::DMS)
            }).collect()
        },
        None => HashSet::with_capacity(0)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_media_file_identity() {
        let base = Path::new("Music");

        // test bounce to lowercase
        assert_eq!(
            "andrew w. k./i get wet/02 - party hard.mp3",
            LibraryFile::new(
                &PathBuf::from("Music/Andrew W. K./I Get Wet/02 - Party Hard.mp3"),
                base,
                LibrarySource::Local
            ).id
        );
        // test supported ascii characters
        assert_eq!(
            "mêlée/everyday behavior/01 - got it all.mp3",
            LibraryFile::new(
                &PathBuf::from("Music/Mêlée/Everyday Behavior/01 - Got It All.mp3"),
                base,
                LibrarySource::Local
            ).id
        );
        // test strip colons
        assert_eq!(
            "apocalyptica/begin- again/01 - track- thing.mp3",
            LibraryFile::new(
                &PathBuf::from("Music/Apocalyptica/Begin: Again/01 - Track: Thing.mp3"),
                base,
                LibrarySource::Local
            ).id
        );
        // test strip tabs
        assert_eq!(
            "theend/something.mp3",
            LibraryFile::new(
                &PathBuf::from("Music/The\tEnd/Something.mp3"),
                base,
                LibrarySource::DMS
            ).id
        );
    }
}
