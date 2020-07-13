use std::path::Path;

use super::*;

use std::mem::size_of;

#[test]
fn test_get_artist_flac() {
    // should return default artist on blank flac
    assert_eq!(
        DEFAULT_ARTIST,
        get_artist_flac(Path::new("test/fixtures/flac/blank.flac")).unwrap()
    );
    assert_eq!(
        "Album Artist",
        get_artist_flac(Path::new("test/fixtures/flac/albumartist.flac")).unwrap()
    );
    assert_eq!(
        "Artist",
        get_artist_flac(Path::new("test/fixtures/flac/artist.flac")).unwrap()
    );
    assert_eq!(
        "Composer",
        get_artist_flac(Path::new("test/fixtures/flac/composer.flac")).unwrap()
    );
}

#[test]
fn test_get_album_flac() {
    // should return default album on blank flac
    assert_eq!(
        DEFAULT_ALBUM,
        get_album_flac(Path::new("test/fixtures/flac/blank.flac")).unwrap()
    );
    assert_eq!(
        "Album",
        get_album_flac(Path::new("test/fixtures/flac/album.flac")).unwrap()
    );
}

#[test]
fn test_get_genre_flac() {
    // should return default genre on blank flac
    assert_eq!(
        DEFAULT_GENRE,
        get_genre_flac(Path::new("test/fixtures/flac/blank.flac")).unwrap()
    );
    assert_eq!(
        "Genre",
        get_genre_flac(Path::new("test/fixtures/flac/genre.flac")).unwrap()
    );
    assert_eq!(
        "Style 1",
        get_genre_flac(Path::new("test/fixtures/flac/style.flac")).unwrap()
    );
}

#[test]
#[should_panic(expected = "does not contain an id3 tag")]
fn test_get_artist_id3_empty() {
    // this file has no id3 header; should fail
    get_artist_id3(&id3::Tag::read_from_path(Path::new("test/fixtures/id3/no-id3.mp3")).unwrap());
}

#[test]
fn test_get_artist_id3() {
    // test that the file without any tags returns DEFAULT_ARTIST
    // FIXME why did this ever work?
    // assert_eq!(
    //     DEFAULT_ARTIST,
    //     get_artist_id3(
    //         &id3::Tag::read_from_path(Path::new("test/fixtures/id3/blank.mp3")).unwrap()
    //     )
    // );
    // test that it finds the value in TPE1
    assert_eq!(
        "Tee-Pee 1",
        get_artist_id3(&id3::Tag::read_from_path(Path::new("test/fixtures/id3/tpe1.mp3")).unwrap())
    );
    // test that it finds the value in TPE2
    assert_eq!(
        "Tee-Pee 2",
        get_artist_id3(&id3::Tag::read_from_path(Path::new("test/fixtures/id3/tpe2.mp3")).unwrap())
    );
    // test that it finds the value in TOPE

    assert_eq!(
        "Tope Rope",
        get_artist_id3(&id3::Tag::read_from_path(Path::new("test/fixtures/id3/tope.mp3")).unwrap())
    );
}

#[test]
#[should_panic(expected = "does not contain an id3 tag")]
fn test_get_album_id3_empty() {
    // this file has no id3 header; should fail
    get_album_id3(&id3::Tag::read_from_path(Path::new("test/fixtures/id3/no-id3.mp3")).unwrap());
}

#[test]
fn test_get_album_id3() {
    // test that the file without any tags returns DEFAULT_ALBUM
    assert_eq!(
        DEFAULT_ALBUM,
        get_album_id3(&id3::Tag::read_from_path(Path::new("test/fixtures/id3/blank.mp3")).unwrap())
    );
    // test that it finds the value in TALB
    assert_eq!(
        "The Album",
        get_album_id3(&id3::Tag::read_from_path(Path::new("test/fixtures/id3/talb.mp3")).unwrap())
    );
}

#[test]
#[should_panic(expected = "does not contain an id3 tag")]
fn test_get_genre_id3_empty() {
    // this file has no id3 header; should fail
    get_genre_id3(&id3::Tag::read_from_path(Path::new("test/fixtures/id3/no-id3.mp3")).unwrap());
}

#[test]
fn test_get_genre_id3() {
    // test that the file without any tags returns DEFAULT_GENRE
    assert_eq!(
        DEFAULT_GENRE,
        get_genre_id3(&id3::Tag::read_from_path(Path::new("test/fixtures/id3/blank.mp3")).unwrap())
    );
    // test that it finds the value in TCON
    assert_eq!(
        "Metalstep",
        get_genre_id3(&id3::Tag::read_from_path(Path::new("test/fixtures/id3/tcon.mp3")).unwrap())
    );
}

#[test]
#[ignore]
fn test_get_track_number_id3_empty() {
    panic!("not implemented")
}

#[test]
#[ignore]
fn test_get_track_number_id3() {
    panic!("not implemented")
}

#[test]
#[ignore]
fn test_get_duration_mp3() {
    panic!("not implemented")
}
