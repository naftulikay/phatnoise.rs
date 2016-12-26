extern crate walkdir;

use walkdir::{DirEntry, WalkDir};

fn main() {
    // so tempted to write let walker = TexasRanger
    let walker = WalkDir::new("/home/naftuli/Music").min_depth(1).into_iter();

    // find them
    for entry in walker.filter_map(|e| e.ok()).filter(|e| is_media_file(e)) {
        println!("{}", entry.path().display());
    }
}

fn is_media_file(entry: &DirEntry) -> bool {
    entry.path().is_file() && entry.path().to_str().map(|s| s.ends_with(".mp3")).unwrap_or(false)
}
