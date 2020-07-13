use libc::futimens;
use libc::time_t;
use libc::timespec;

use std::error::Error;
use std::fmt;
use std::fs;
use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::path::Path;
use std::time::UNIX_EPOCH;

#[derive(Debug)]
pub struct ModTimeUpdateError {
    rc: isize,
}

impl Error for ModTimeUpdateError {
    fn description(&self) -> &str {
        "Unable to call futimens without error"
    }
}

impl fmt::Display for ModTimeUpdateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to call futimens, return code {}", self.rc)
    }
}

// Copy the modified time of source to dest
#[cfg(target_os = "linux")]
pub fn copy_mtime(source: &Path, dest: &Path) -> Result<(), ModTimeUpdateError> {
    let smeta = fs::metadata(source).unwrap();
    let s_modified = smeta.modified().unwrap();

    let dmeta = fs::metadata(dest).unwrap();
    let d_accessed = dmeta.accessed().unwrap();

    let accessed_duration = d_accessed.duration_since(UNIX_EPOCH).unwrap();
    let modified_duration = s_modified.duration_since(UNIX_EPOCH).unwrap();

    let rc = unsafe {
        let file = File::open(dest).unwrap();

        let accessed = timespec {
            tv_sec: accessed_duration.as_secs() as time_t,
            tv_nsec: accessed_duration.subsec_nanos() as libc::c_long,
        };

        let modified = timespec {
            tv_sec: modified_duration.as_secs() as time_t,
            tv_nsec: modified_duration.subsec_nanos() as libc::c_long,
        };

        let times = [accessed, modified];

        futimens(file.as_raw_fd() as libc::c_int, times.as_ptr()) as isize
    };

    if rc == 0 {
        Ok(())
    } else {
        Err(ModTimeUpdateError { rc })
    }
}
