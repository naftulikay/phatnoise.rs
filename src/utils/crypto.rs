use std::fmt;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

#[derive(Eq,PartialEq)]
pub struct Sha256Digest([u8; 32]);

impl fmt::Display for Sha256Digest {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for b in self.0.iter() {
            result += &format!("{:02x}", b);
        }
        write!(f, "{}", result)
    }
}

pub fn sha256sum(path: &Path) -> Result<Sha256Digest,io::Error> {
    let mut f = File::open(path)?;
    let mut buf: [u8; 4096] = [0; 4096];
    let mut digest = Sha256::new();

    while let Ok(bytes_read) = f.read(&mut buf) {
        if bytes_read == 0 {
            break;
        }

        digest.input(&buf[..bytes_read]);
    }

    let mut result: [u8; 32] = [0; 32];
    digest.result(&mut result);

    Ok(Sha256Digest(result))
}
