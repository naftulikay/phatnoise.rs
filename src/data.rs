use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

/// The database index into the tracks CSV file, to be rendered to disk as `tracks.idx`.
pub struct TracksDbIndex {
    pub track_offsets: Vec<u32>,
}

impl TracksDbIndex {
    /// Dump this database index to the given file.
    pub fn write<P: AsRef<Path>>(&self, dest: P) {
        let mut f = BufWriter::new(File::create(dest).expect("unable to create tracks.idx"));

        // first, write the count as a u32 at the beginning of the file
        let count = self.track_offsets.len() as u32;

        f.write(&count.to_le_bytes())
            .expect("unable to write count");

        for offset in &self.track_offsets {
            // write each offset in order into the file
            f.write(&offset.to_le_bytes())
                .expect("unable to write offset");
        }

        // critical to flush always
        f.flush().expect("unable to flush data to disk");
    }
}

pub struct ArtistsDbIndex {
    pub id: u32,
    pub track_offsets: Vec<u32>,
}
