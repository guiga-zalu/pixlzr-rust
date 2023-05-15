use crate::data_types::Semver;

pub const PIXLZR_MAGIC_NUMBERS: &[u8] = b"PIXLZR";
pub const PIXLZR_VERSION: Semver = Semver {
    major: 0,
    minor: 0,
    patch: 1,
};
pub const PIXLZR_MAGIC_VERSION: &[u8] = &[0, 0, 1];
pub const PIXLZR_BLOCK_HEADER: &[u8] = b"block";
pub const PIXLZR_HEADER_SIZE: usize =
    PIXLZR_MAGIC_NUMBERS.len() + PIXLZR_MAGIC_VERSION.len() + 4 * 4 + 1;
