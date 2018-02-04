extern crate phatnoise;

use phatnoise::dms;

fn main() {
    println!("DMS: is present? {}; device: {:?}; is mounted? {}; mount: {:?}",
        dms::is_dms_present(),
        dms::get_dms_device().expect("No DMS device found."),
        dms::is_dms_mounted(),
        dms::get_dms_mount_point().expect("DMS not mounted.")
    );
}
