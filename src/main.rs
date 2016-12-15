extern crate id3;

use id3::Tag;

fn main() {
    let tag = Tag::read_from_path("test.mp3").unwrap();
    // get either the album artist, or the artist, or return "unknown artist"
    let artist = tag.album_artist().or(tag.artist()).unwrap_or("Unknown Artist");
    let album = tag.album().unwrap_or("Unknown Album");
    let genre = tag.genre().unwrap_or("Unknown Genre");

    println!("Artist: {}; Album: {}; Genre: {}", artist, album, genre);
}
