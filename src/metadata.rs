#[cfg(test)]
mod test;

use id3;

use log::debug;

use regex::Regex;

use std::cmp::Ordering;
use std::convert::From;
use std::error::Error;
use std::fmt;
use std::iter::Iterator;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use std::string::String;
use std::sync::Arc;
use std::time::Duration;

use metaflac;

use mp3_duration;

use crate::utils::StringPool;

static DEFAULT_ARTIST: &'static str = "Unknown Artist";
static DEFAULT_ALBUM: &'static str = "Unknown Album";
static DEFAULT_GENRE: &'static str = "Unknown Genre";
static DEFAULT_TITLE: &'static str = "Unknown Title";
static DEFAULT_TRACK_NUMBER: &'static str = "0";

lazy_static! {
    static ref TRACK_NUMBER: Regex =
        Regex::new(r"(?i)^(?P<track>\d+)(?:/(?P<total>\d+))?$").unwrap();
    static ref PRONOUN_START: Regex = Regex::new(r"(?i)^(?:a|an|the)\s+").unwrap();
}

pub struct MediaMetadata {
    pub path: PathBuf,
    pub base: PathBuf,
    pub artist: Arc<str>,
    pub album: Arc<str>,
    pub genre: Arc<str>,
    pub title: String,
    pub track_number: u16,
    pub duration: u64,
}

impl MediaMetadata {
    pub fn load(path: &Path, base: &Path, pool: &StringPool) -> Result<Self, MediaParsingError> {
        debug!("Loading metadata from file {}...", path.display());
        match path.extension() {
            Some(extension) if extension == "mp3" => {
                let tag = id3::Tag::read_from_path(path)?;

                Ok(MediaMetadata {
                    path: path.to_path_buf(),
                    base: base.to_path_buf(),
                    artist: pool.get(&get_artist_id3(&tag)),
                    album: pool.get(&get_album_id3(&tag)),
                    genre: pool.get(&get_genre_id3(&tag)),
                    title: get_title_id3(&tag),
                    track_number: get_track_number_id3(&tag),
                    duration: get_duration_mp3(&path).as_secs(),
                })
            }
            _ => Err(MediaParsingError::UnrecognizedFormat),
        }
    }

    /// Returns the track's artist for sorting
    fn artist_sortable(&self) -> &str {
        if PRONOUN_START.is_match(&self.artist) {
            &self.artist[PRONOUN_START.find(&self.artist).unwrap().end()..]
        } else {
            &self.artist
        }
    }

    /// Returns the track's album for sorting
    fn album_sortable(&self) -> &str {
        if PRONOUN_START.is_match(&self.album) {
            &self.album[PRONOUN_START.find(&self.album).unwrap().end()..]
        } else {
            &self.album
        }
    }

    /// Returns the track's genre for sorting
    fn genre_sortable(&self) -> &str {
        if PRONOUN_START.is_match(&self.genre) {
            &self.genre[PRONOUN_START.find(&self.genre).unwrap().end()..]
        } else {
            &self.genre
        }
    }

    /// Returns the track's title for sorting
    fn title_sortable(&self) -> &str {
        if PRONOUN_START.is_match(&self.title) {
            &self.title[PRONOUN_START.find(&self.title).unwrap().end()..]
        } else {
            &self.title
        }
    }

    /// Sorting utility for sorting by artist, album, track number, track title, and by genre in the
    /// given order.
    pub fn by_artist(this: &Self, that: &Self) -> Ordering {
        this.artist_sortable()
            .cmp(that.artist_sortable())
            .then(this.album_sortable().cmp(that.album_sortable()))
            .then(this.track_number.cmp(&that.track_number))
            .then(this.title_sortable().cmp(that.title_sortable()))
            .then(this.genre_sortable().cmp(that.genre_sortable()))
    }

    /// Sorting utility for sorting by genre, artist, album, track number, and by track title in the
    /// given order.
    pub fn by_genre(this: &Self, that: &Self) -> Ordering {
        this.genre_sortable()
            .cmp(that.genre_sortable())
            .then(this.artist_sortable().cmp(that.artist_sortable()))
            .then(this.album_sortable().cmp(that.album_sortable()))
            .then(this.track_number.cmp(&that.track_number))
            .then(this.title_sortable().cmp(that.title_sortable()))
    }

    /// Get the location of a file relative to the DMS root.
    fn dms_location(&self) -> PathBuf {
        Path::new("/dos/data").join(self.path.strip_prefix(&self.base).unwrap())
    }

    /// Converts a MediaMetadata instance into the CSV format expected by PhatNoise for artists,
    /// albums, genres, and tracks databases.
    ///
    /// See: https://github.com/naftulikay/phatnoise.rs/wiki/Sync-Workflow
    pub fn to_csv(&self) -> String {
        format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            self.dms_location().display(),
            self.title,
            self.artist,
            self.album,
            self.genre,
            self.duration,
            "",
            self.track_number,
            "NotFound.jpg"
        )
    }
}

impl fmt::Display for MediaMetadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_csv())
    }
}

#[derive(Debug)]
pub enum MediaParsingError {
    FLACError { err: metaflac::Error },
    ID3Error { err: id3::Error },
    UnrecognizedFormat,
}

impl Error for MediaParsingError {
    fn description(&self) -> &str {
        "Unable to load metadata from media file."
    }
}

impl fmt::Display for MediaParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &MediaParsingError::FLACError { ref err } => {
                write!(f, "Unable to load metadata from media file: {}", err)
            }
            &MediaParsingError::ID3Error { ref err } => {
                write!(f, "Unable to load metadata from media file: {}", err)
            }
            &MediaParsingError::UnrecognizedFormat => write!(
                f,
                "Unable to load metadata from media file, unrecognized format."
            ),
        }
    }
}

impl From<id3::Error> for MediaParsingError {
    fn from(e: id3::Error) -> Self {
        MediaParsingError::ID3Error { err: e }
    }
}

fn get_artist_flac(path: &Path) -> Result<String, metaflac::Error> {
    let tag = metaflac::Tag::read_from_path(path)?;

    for tag_name in &["ALBUMARTIST", "ARTIST", "COMPOSER"] {
        // FIXME this is garbage horse trash
        if let Some(entities) = tag.get_vorbis(tag_name) {
            for entity in entities {
                if entity.len() > 0 {
                    return Ok(entity.to_string());
                }
            }
        }
    }

    Ok(DEFAULT_ARTIST.to_string())
}

fn get_album_flac(path: &Path) -> Result<String, metaflac::Error> {
    let tag = metaflac::Tag::read_from_path(path)?;
    let metadata = tag.vorbis_comments().unwrap();

    // find the first non-empty tag, or return DEFAULT_ALBUM
    Ok(metadata
        .album()
        .iter()
        .flat_map(|&v| v)
        .map(|s| s.to_string())
        .filter(|f| f.len() > 0)
        .nth(0)
        .unwrap_or(DEFAULT_ALBUM.to_string()))
}

fn get_genre_flac(path: &Path) -> Result<String, metaflac::Error> {
    let tag = metaflac::Tag::read_from_path(path)?;

    for tag_name in &["GENRE", "STYLE"] {
        // FIXME this is garbage horse trash
        if let Some(entities) = tag.get_vorbis(tag_name) {
            for entity in entities {
                if entity.len() > 0 {
                    return Ok(entity.to_string());
                }
            }
        }
    }

    Ok(DEFAULT_GENRE.to_string())
}

fn get_artist_id3(tag: &id3::Tag) -> String {
    // leeched from here: id3.org/id3v2.4.0-frames
    /*
    TPE2: Used by players as "album artist," the artist who created the album of this track.
    TPE1: Used by players as "artist," the artist who performed the track on the given album
    TOPE: Original artist.
    */
    // FIXME yikes, could be better
    tag.album_artist()
        .unwrap_or(
            tag.artist().unwrap_or(
                tag.get("TOPE")
                    .map(|frame| frame.content().text().unwrap_or(DEFAULT_ARTIST))
                    .unwrap_or(DEFAULT_ARTIST),
            ),
        )
        .to_string()
}

fn get_album_id3(tag: &id3::Tag) -> String {
    tag.album().unwrap_or(DEFAULT_ALBUM).to_string()
}

fn get_genre_id3(tag: &id3::Tag) -> String {
    tag.genre().unwrap_or(DEFAULT_GENRE).to_string()
}

fn get_track_number_id3(tag: &id3::Tag) -> u16 {
    // find the first non-empty tag or return DEFAULT_TRACK_NUMBER
    let track = tag
        .track()
        .map(|i| format!("{}", i))
        .unwrap_or(DEFAULT_TRACK_NUMBER.to_string());

    // the above value is a string of the format (\d+)(?:/(\d+))?, deconstruct and parse into an int
    TRACK_NUMBER
        .captures(&track)
        .unwrap()
        .name("track")
        .map(|s| u16::from_str(s.as_str()).unwrap())
        .unwrap()
}

fn get_title_id3(tag: &id3::Tag) -> String {
    tag.title().unwrap_or(DEFAULT_TITLE).to_string()
}

fn get_duration_mp3(path: &Path) -> Duration {
    // some of my tracks panic when trying to get the duration, so failover to 0
    mp3_duration::from_path(path).unwrap_or(Duration::new(0, 0))
}
