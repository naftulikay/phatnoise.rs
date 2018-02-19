use rayon::prelude::*;

use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::iter::Iterator;
use std::path::Path;
use std::process;

use dms;
use library::LibraryFile;
use library::LibrarySource;
use library::get_local_media_library;
use library::get_dms_media_library;
use utils::crypto::sha256sum;
use utils::fs::copy_mtime;

pub fn synchronize() {
    if !dms::is_dms_present() {
        error!("No DMS device detected.");
        process::exit(1);
    }

    if !dms::is_dms_mounted() {
        error!("DMS device is present but not mounted.");
        process::exit(1);
    }

    synchronize_media_files();
}

pub fn synchronize_media_files() {
    info!("Synchronizing media files with DMS...");

    let local_dir = Path::join(Path::new(&match env::var("HOME") {
        Ok(value) => value,
        Err(e) => {
            error!("Unable to detect home directory: {}", e);
            process::exit(1);
        }
    }), Path::new("Music"));

    let dms_dir = dms::get_dms_mount_point().expect("DMS not present or not mounted.");

    debug!("Music directory: {}", local_dir.display());

    // load a list of files from the local media library and from the DMS
    let (local, dms) = (get_local_media_library(&local_dir), get_dms_media_library());
    // use hardcore HashSet intersections to detect what is new, changed, and deleted
    let (added, deleted, changed) = (added_files(&local, &dms), deleted_files(&local, &dms),
        changed_files(&local, &dms));

    // copy new files
    info!("Copying {} new files to the DMS...", added.len());
    copy_files(&added, &local, &dms_dir);

    // copy updated files
    info!("Copying {} changed files to the DMS...", changed.len());
    copy_files(&changed, &local, &dms_dir);

    // delete removed files
    info!("Deleting {} orphaned files from the DMS...", deleted.len());
    delete_files(&deleted);
}

fn copy_files(files: &Vec<&LibraryFile>, local: &BTreeSet<LibraryFile>, dms_dir: &Path) {
    for file in files {
        let (source, dest) = (&local.get(file).unwrap().path, Path::join(&dms_dir, Path::new(&file.debase())));
        debug!("Copying local file {} to DMS at {}...", source.display(), dest.display());

        let dest_dir = &dest.parent().expect(
            format!("Unable to get parent directory for {}", dest.display()).as_str()
        );

        // create parent directory for file
        if !dest_dir.is_dir() {
            debug!("Creating parent directory {}", dest_dir.display());
            fs::create_dir_all(&dest_dir).expect(
                format!("Unable to create parent directory for {}", dest.display()).as_str()
            );
        }

        // copy file
        fs::copy(&source, &dest).expect(format!("Unable to copy file {} to DMS", source.display()).as_str());
        // update modification time
        copy_mtime(&source, &dest).expect(format!("Unable to copy modification time from source to destination {}", dest.display()).as_str());
    }
}

fn delete_files(files: &Vec<&LibraryFile>) {
    // we find all files in the list that are explicitly on the DMS to be safe
    for file in files.iter().filter(|f| f.source == LibrarySource::DMS).map(|f| &f.path) {
        debug!("Deleting orphaned file from DMS {}", file.display());
        fs::remove_file(file).expect(format!("Unable to remove file from DMS: {}", file.display()).as_str());
    }
}


/// Retrieve a list of new files to be copied to the DMS.
pub fn added_files<'a>(local: &'a BTreeSet<LibraryFile>, dms: &'a BTreeSet<LibraryFile>) ->
        Vec<&'a LibraryFile> {
    local.difference(dms).collect()
}

/// Retrieve a list of deleted files to be removed from the DMS.
pub fn deleted_files<'a>(local: &'a BTreeSet<LibraryFile>, dms: &'a BTreeSet<LibraryFile>) ->
        Vec<&'a LibraryFile> {
    dms.difference(local).collect()
}

/// Retrieve a list of changed files to be updated on the DMS.
pub fn changed_files<'a>(local: &'a BTreeSet<LibraryFile>, dms: &'a BTreeSet<LibraryFile>) ->
        Vec<&'a LibraryFile> {
    local.into_par_iter().filter(|p| {
        if !dms.contains(p) {
            // if the DMS does not have the file, omit it
            return false
        }

        let (local, remote) = (p, dms.get(*p).unwrap());
        let (lmeta, rmeta) = (fs::metadata(&local.path).unwrap(), fs::metadata(&remote.path).unwrap());
        let (llen, rlen) = (lmeta.len(), rmeta.len());
        let (lmod, rmod) = (lmeta.modified().unwrap(), rmeta.modified().unwrap());

        if llen != rlen {
            // if the size doesn't match, always taint
            debug!("{}: changed - size not equal", local.debase());
            return true
        }

        let (first, last) = (lmod.min(rmod), lmod.max(rmod));
        let diff = last.duration_since(first).unwrap();

        if diff.as_secs() <= 3 {
            // if the size matches and the modified time difference is less than or equal to 3s
            return false
        }

        // now we have a situation where the size is equal but the modified time is off
        // to correct this issue, we hash both the source and destination. if the source and dest
        // have the same checksum, we update the remote mtime to equal the local mtime
        let (source, destination) = (
            sha256sum(&local.path).expect("unable to compute checksum for local file"),
            sha256sum(&remote.path).expect("unable to compute checksum for remote file")
        );

        if source == destination {
            // the hashes map, so let's copy the modification time from local to remote to resolve
            // future comparisons
            debug!("{}: unchanged - checksums match", local.debase());
            copy_mtime(&local.path, &remote.path).ok();

            // checksums matched, so we're done with this file
            false
        } else {
            // checksums differed, so we must mark dirty
            debug!("{}: changed - checksums differ", local.debase());
            true
        }
    }).collect()
}
