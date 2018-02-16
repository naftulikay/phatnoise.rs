use regex::Regex;

use std::fs;

use std::io;
use std::io::BufRead;

use std::path::{
    Path,
    PathBuf,
};

lazy_static! {
    static ref PROC_MOUNT_LINE: Regex = Regex::new(
        r"(?i)^(?P<device>[^\s]+)\s+(?P<mount>[^\s]+)"
    ).unwrap();
}

pub fn is_dms_present() -> bool {
    get_dms_device().is_some()
}

#[cfg(target_os="linux")]
pub fn get_dms_device() -> Option<PathBuf> {
    fs::canonicalize(Path::new("/dev/disk/by-label/PHTDTA")).ok()
}

pub fn is_dms_mounted() -> bool {
    get_dms_mount_point().is_some()
}

#[cfg(target_os="linux")]
pub fn get_dms_mount_point() -> Option<PathBuf> {
    if !is_dms_present() {
        return None
    }

    let device = get_dms_device()?;
    let f = fs::File::open(Path::new("/proc/mounts")).ok()?;
    let buffer = io::BufReader::new(f);

    for line in buffer.lines().filter_map(|s| s.ok()).collect::<Vec<String>>() {
        if line.starts_with(device.to_str()?) {
            return PROC_MOUNT_LINE.captures(&line)?.name("mount")
                .map(|s| PathBuf::from(s.as_str()));
        }
    }

    None
}
