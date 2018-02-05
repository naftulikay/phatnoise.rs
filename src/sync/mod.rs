use rayon::prelude::*;

use std::collections::HashSet;
use std::fs;

use library::LibraryFile;
use utils::crypto::sha256sum;
use utils::fs::copy_mtime;

/// Retrieve a list of new files to be copied to the DMS.
pub fn added_files<'a>(local: &'a HashSet<LibraryFile>, dms: &'a HashSet<LibraryFile>) ->
        Vec<&'a LibraryFile> {
    local.difference(dms).collect()
}

/// Retrieve a list of deleted files to be removed from the DMS.
pub fn deleted_files<'a>(local: &'a HashSet<LibraryFile>, dms: &'a HashSet<LibraryFile>) ->
        Vec<&'a LibraryFile> {
    dms.difference(local).collect()
}

/// Retrieve a list of changed files to be updated on the DMS.
pub fn changed_files<'a>(local: &'a HashSet<LibraryFile>, dms: &'a HashSet<LibraryFile>) ->
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
