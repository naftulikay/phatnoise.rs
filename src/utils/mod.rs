pub mod crypto;
pub mod fs;
pub mod media;
pub mod stringpool;

#[cfg(test)]
mod test;

// export
pub use utils::stringpool::StringPool;

pub static FAT32_DELETE_CHARS: &'static [char] = &['\t'];
pub static FAT32_HYPHENIZE_CHARS: &'static [char] = &[':'];
