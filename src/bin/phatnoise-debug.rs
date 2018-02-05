extern crate phatnoise;

use phatnoise::dms;
use phatnoise::library;
use phatnoise::library::LibraryFile;

use std::path::PathBuf;

fn main () {
    debug_dms();
    debug_library();
}

fn debug_library() {
    let local_library = library::get_local_media_library(&PathBuf::from("/home/naftuli/Music"));
    let dms_library = library::get_dms_media_library();

    let new_files: Vec<&LibraryFile> = local_library.difference(&dms_library).collect();
    let delete_files: Vec<&LibraryFile> = dms_library.difference(&local_library).collect();

    println!("Library: Local Files: {}; DMS Files: {}", local_library.len(), dms_library.len());
    println!("Files on Local But Not DMS: {}", new_files.len());
    println!("Files on DMS But Not Local: {}", delete_files.len());

    for new_local in new_files {
        println!("NEW FILE: {:?}", new_local.path);
    }

    for delete_remote in delete_files {
        println!("DELETE FILE: {:?}", delete_remote.path);
    }
}

fn debug_dms() {
    let dms_present = dms::is_dms_present();
    let dms_mounted = dms::is_dms_mounted();

    let dms_device = match dms::get_dms_device() {
        Some(pathbuf) => pathbuf.to_string_lossy().into_owned(),
        None => "(null)".to_owned(),
    };

    let dms_mountpoint = match dms::get_dms_mount_point() {
        Some(pathbuf) => String::from(pathbuf.to_string_lossy()),
        None => String::from("(null)")
    };

    println!("DMS: is present? {}; device: {}; is mounted? {}; mountpoint: {}",
        dms_present, dms_device, dms_mounted, dms_mountpoint,
    );
}
