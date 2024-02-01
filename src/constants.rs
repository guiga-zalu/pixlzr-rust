use crate::data_types::Semver;

pub const OCTETS_F32: usize = 4;
pub const OCTETS_U32: usize = 4;

pub const QOI_HEADER_SIZE: usize = 14;
pub const QOI_MAGIC: &[u8] = b"qoif";
pub const QOI_MAGIC_SIZE: usize = QOI_MAGIC.len();

pub const PIXLZR_MAGIC_NUMBERS: &[u8] = b"PIXLZR";
pub const PIXLZR_MAGIC_VERSION: &[u8] = &[0, 0, 2];

pub const PIXLZR_VERSION: Semver = Semver {
	major: 0,
	minor: 0,
	patch: 2,
};

pub const PIXLZR_HEADER_SIZE: usize =
	PIXLZR_MAGIC_NUMBERS.len() + PIXLZR_MAGIC_VERSION.len() + 4 * 4 + 1;

pub const PIXLZR_BLOCK_MAGIC_NUMBERS: &[u8] = b"block";
pub const PIXLZR_BLOCK_HEADER_BASE_SIZE: usize =
	PIXLZR_BLOCK_MAGIC_NUMBERS.len() + OCTETS_F32 + OCTETS_U32;
/// -
/// ```txt
/// = The header's magic numbers (&[u8])
/// + the block value (f32)
/// + the encoded block size (u32)
/// + the QOI header (&[u8])
/// - the QOI magic numbers (&[u8])
/// ```
pub const PIXLZR_BLOCK_HEADER_SIZE: usize =
	PIXLZR_BLOCK_HEADER_BASE_SIZE + QOI_HEADER_SIZE - QOI_MAGIC.len();
