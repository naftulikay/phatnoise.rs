use std::hash::{Hash, Hasher};
use std::path::Path;
use std::string::String;

// FIXME base needs to be an Arc, only one should exist in memory for a set of MediaFile structs
pub struct MediaFile<'a> {
    path: &'a Path,
    base: &'a Path
}

impl<'a> MediaFile<'a> {

    pub fn new(path: &'a Path, base: &'a Path) -> MediaFile<'a> {
        MediaFile{ path: path, base: base }
    }

    fn id(&self) -> String {
        self.path.strip_prefix(self.base).unwrap().to_str().unwrap().to_lowercase()
    }
}

impl<'a> Hash for MediaFile<'a> {

    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}

#[test]
fn test_media_file_identity() {
    let f = MediaFile::new(
        Path::new("/home/naftuli/Music/Andrew W. K./I Get Wet/02 - Party Hard.mp3"),
        Path::new("/home/naftuli/Music")
    );

    assert_eq!("andrew w. k./i get wet/02 - party hard.mp3", format!("{}", f.id()));
}
