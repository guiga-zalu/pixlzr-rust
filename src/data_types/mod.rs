pub(crate) mod block;
#[cfg(feature = "image-rs")]
pub(crate) mod iter;
pub(crate) mod pixlzr;
pub mod semver;
pub use self::{block::*, iter::*, pixlzr::*, semver::Semver};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
#[cfg(feature = "cli")]
#[derive(clap::ValueEnum)]
pub enum FilterType {
	/// Nearest Neighbor
	#[default]
	Nearest = 0,

	/// Linear Filter
	Triangle = 1,

	/// Cubic Filter
	CatmullRom = 2,

	/// Gaussian Filter
	Gaussian = 3,

	/// Lanczos with window 3
	Lanczos3 = 4,
}

#[cfg(feature = "image-rs")]
use image::imageops::FilterType as ImageFilterType;
#[cfg(feature = "image-rs")]
impl From<FilterType> for ImageFilterType {
	fn from(value: FilterType) -> Self {
		match value {
			FilterType::Nearest => ImageFilterType::Nearest,
			FilterType::Triangle => ImageFilterType::Triangle,
			FilterType::CatmullRom => ImageFilterType::CatmullRom,
			FilterType::Gaussian => ImageFilterType::Gaussian,
			FilterType::Lanczos3 => ImageFilterType::Lanczos3,
		}
	}
}

impl From<u8> for FilterType {
	fn from(value: u8) -> Self {
		match value {
			0 => FilterType::Nearest,
			1 => FilterType::Triangle,
			2 => FilterType::CatmullRom,
			3 => FilterType::Gaussian,
			4 => FilterType::Lanczos3,
			_ => FilterType::Nearest,
		}
	}
}
