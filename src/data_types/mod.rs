pub(crate) mod block;
#[cfg(feature = "image-rs")]
pub(crate) mod iter;
pub(crate) mod pixlzr;
#[cfg(feature = "image-rs")]
pub(crate) mod pixlzr_image;
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
use image::imageops::FilterType as I_FilterType;
#[cfg(feature = "image-rs")]
impl From<FilterType> for I_FilterType {
	fn from(value: FilterType) -> Self {
		match value {
			FilterType::Nearest => I_FilterType::Nearest,
			FilterType::Triangle => I_FilterType::Triangle,
			FilterType::CatmullRom => I_FilterType::CatmullRom,
			FilterType::Gaussian => I_FilterType::Gaussian,
			FilterType::Lanczos3 => I_FilterType::Lanczos3,
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
