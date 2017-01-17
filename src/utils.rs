#[cfg(test)]
mod test;

pub mod crypto;
pub mod fs;
pub mod media;
pub mod stringpool;

// export
pub use crate::utils::stringpool::StringPool;

pub static FAT32_DELETE_CHARS: &'static [char] = &['\t'];
pub static FAT32_HYPHENIZE_CHARS: &'static [char] = &[':'];
