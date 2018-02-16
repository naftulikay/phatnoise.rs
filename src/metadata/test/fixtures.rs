use id3::Tag as ID3Tag;

use metaflac::Tag as FLACTag;

#[test]
fn test_flac_blank() {
    // blank.flac should have no tags
    assert_eq!(0, FLACTag::read_from_path("test/fixtures/flac/blank.flac").unwrap()
        .vorbis_comments().unwrap().comments.len());
}

#[test]
fn test_flac_albumartist() {
    let tag = FLACTag::read_from_path("test/fixtures/flac/albumartist.flac").unwrap();
    let metadata = tag.vorbis_comments().unwrap();
    assert_eq!("Album Artist", metadata.album_artist().unwrap()[0]);
    assert_eq!("Artist", metadata.artist().unwrap()[0]);
    // metaflac has a case-sensitivity bug as of 0.1.5: https://github.com/jameshurst/rust-metaflac/issues/2
    assert_eq!("Composer", tag.get_vorbis("COMPOSER").iter().flat_map(|&v| v).nth(0).unwrap());
}

#[test]
fn test_flac_artist() {
    let tag = FLACTag::read_from_path("test/fixtures/flac/artist.flac").unwrap();
    let metadata = tag.vorbis_comments().unwrap();
    assert!(metadata.album_artist().is_none());
    assert_eq!("Artist", metadata.artist().unwrap()[0]);
    assert_eq!("Composer", tag.get_vorbis("COMPOSER").iter().flat_map(|&v| v).nth(0).unwrap());
}

#[test]
fn test_flac_composer() {
    let tag = FLACTag::read_from_path("test/fixtures/flac/composer.flac").unwrap();
    let metadata = tag.vorbis_comments().unwrap();
    assert!(metadata.album_artist().is_none());
    assert!(metadata.artist().is_none());
    assert_eq!("Composer", tag.get_vorbis("COMPOSER").iter().flat_map(|&v| v).nth(0).unwrap());
}

#[test]
fn test_flac_album() {
    let tag = FLACTag::read_from_path("test/fixtures/flac/album.flac").unwrap();
    let metadata = tag.vorbis_comments().unwrap();
    assert_eq!("Album", metadata.album().unwrap()[0]);
}

#[test]
fn test_flac_genre() {
    let tag = FLACTag::read_from_path("test/fixtures/flac/genre.flac").unwrap();
    let metadata = tag.vorbis_comments().unwrap();
    assert_eq!("Genre", metadata.genre().unwrap()[0]);
    assert_eq!("Style 1", tag.get_vorbis("STYLE").iter().flat_map(|&v| v).nth(0).unwrap());
    assert_eq!("Style 2", tag.get_vorbis("STYLE").iter().flat_map(|&v| v).nth(1).unwrap());
}

#[test]
fn test_flac_style() {
    let tag = FLACTag::read_from_path("test/fixtures/flac/style.flac").unwrap();
    let metadata = tag.vorbis_comments().unwrap();
    assert!(metadata.genre().is_none());
    assert_eq!("Style 1", tag.get_vorbis("STYLE").iter().flat_map(|&v| v).nth(0).unwrap());
    assert_eq!("Style 2", tag.get_vorbis("STYLE").iter().flat_map(|&v| v).nth(1).unwrap());
}

#[test]
fn test_id3_blank() {
    // blank.mp3 should have absolutely no tags present
    assert_eq!(0, ID3Tag::read_from_path("test/fixtures/id3/blank.mp3").unwrap().frames().len());
}

#[test]
fn test_id3_tpe1() {
    let tags = ID3Tag::read_from_path("test/fixtures/id3/tpe1.mp3").unwrap();
    assert!(tags.get("TPE2").is_none());
    assert_eq!(Some("Tee-Pee 1"), tags.get("TPE1").unwrap().content.text());
    assert_eq!(Some("Tope Rope"), tags.get("TOPE").unwrap().content.text());
}

#[test]
fn test_id3_tpe2() {
    let tags = ID3Tag::read_from_path("test/fixtures/id3/tpe2.mp3").unwrap();
    assert_eq!(Some("Tee-Pee 1"), tags.get("TPE1").unwrap().content.text());
    assert_eq!(Some("Tee-Pee 2"), tags.get("TPE2").unwrap().content.text());
    assert_eq!(Some("Tope Rope"), tags.get("TOPE").unwrap().content.text());
}

#[test]
fn test_id3_tope() {
    let tags = ID3Tag::read_from_path("test/fixtures/id3/tope.mp3").unwrap();
    assert!(tags.get("TPE1").is_none());
    assert!(tags.get("TPE2").is_none());
    assert_eq!(Some("Tope Rope"), tags.get("TOPE").unwrap().content.text());
}

#[test]
fn test_id3_talb() {
    let tags = ID3Tag::read_from_path("test/fixtures/id3/talb.mp3").unwrap();
    assert_eq!(Some("The Album"), tags.get("TALB").unwrap().content.text());
}

#[test]
fn test_id3_tcon() {
    let tags = ID3Tag::read_from_path("test/fixtures/id3/tcon.mp3").unwrap();
    assert_eq!(Some("Metalstep"), tags.get("TCON").unwrap().content.text());
}
