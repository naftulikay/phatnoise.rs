extern crate phatnoise;

use phatnoise::fsync::get_local_media_library;

fn main() {
    let library = get_local_media_library("/home/naftuli/Music");

    println!("Local library has {} files.", library.len());

    for f in library.into_iter().take(5) {
        println!("{} (full: {})", f.id(), f.path.display());
    }
}
