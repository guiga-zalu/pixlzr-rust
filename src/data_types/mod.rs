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

#[cfg(feature = "fir")]
use fast_image_resize::{FilterType as FIR_FilterType, ResizeAlg};
#[cfg(feature = "fir")]
impl From<FIR_FilterType> for FilterType {
	fn from(value: FIR_FilterType) -> Self {
		match value {
			FIR_FilterType::Box => FilterType::Nearest,
			FIR_FilterType::Bilinear => FilterType::Triangle,
			FIR_FilterType::CatmullRom => FilterType::CatmullRom,
			FIR_FilterType::Gaussian => FilterType::Gaussian,
			FIR_FilterType::Lanczos3 => FilterType::Lanczos3,
			_ => FilterType::Nearest,
		}
	}
}

impl FilterType {
	#[cfg(feature = "fir")]
	pub fn to_fir_resizing_algorithm(
		self,
		upscale: bool,
		multiplicity: u8,
	) -> ResizeAlg {
		match self {
			Self::Nearest => ResizeAlg::Nearest,
			f if upscale => match f {
				Self::Triangle => ResizeAlg::SuperSampling(
					FIR_FilterType::Bilinear,
					multiplicity,
				),
				Self::Lanczos3 => ResizeAlg::SuperSampling(
					FIR_FilterType::Lanczos3,
					multiplicity,
				),
				Self::Gaussian => ResizeAlg::SuperSampling(
					FIR_FilterType::Gaussian,
					multiplicity,
				),
				Self::CatmullRom => ResizeAlg::SuperSampling(
					FIR_FilterType::CatmullRom,
					multiplicity,
				),
				_ => unreachable!(),
			},
			f => match f {
				Self::Triangle => {
					ResizeAlg::Convolution(FIR_FilterType::Hamming)
				}
				Self::Lanczos3 => {
					ResizeAlg::Convolution(FIR_FilterType::Lanczos3)
				}
				Self::Gaussian => {
					ResizeAlg::Convolution(FIR_FilterType::Gaussian)
				}
				Self::CatmullRom => {
					ResizeAlg::Convolution(FIR_FilterType::CatmullRom)
				}
				_ => unreachable!(),
			},
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
