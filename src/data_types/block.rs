use std::slice::ChunksExact;

/// ImageBlock
/// - x: u32
/// - y: u32
/// - block: PixlzrBlock
///   + Raw: PixlzrBlockRaw
///     - width: u32
///     - height: u32
///     - block_value: Option<f32>
///     - data: RawImage
///       - alpha: bool
///       - width: u32
///       - height: u32
///       - data: Vec<u8>
///   + Image: PixlzrBlockImage
///     - width: u32
///     - height: u32
///     - block_value: Option<f32>
///     - data: DynamicImage
///
/// PixlzrBlockRaw <-> PixlzrBlockImage
/// - PixlzrBlock(Image) -> PixlzrBlockImage
/// - PixlzrBlock(Raw) -> PixlzrBlockImage
///
/// - PixlzrBlock(Raw) -> PixlzrBlockRaw
/// - PixlzrBlock(Image) -> PixlzrBlockRaw
/// PixlzrBlock.into() -> PixlzrBlock::{Raw, Image}

#[cfg(feature = "fir")]
use fast_image_resize::{
	FilterType as FIR_FilterType, PixelType as FIR_PixelType, ResizeAlg,
	Resizer,
};

#[cfg(feature = "image-rs")]
use image::{
	imageops::FilterType as I_FilterType, DynamicImage, RgbImage,
	RgbaImage,
};

/// Image block representation, with:
/// - `x: u32, y: u32` as the coordinates of the block
/// - `block: PixlzrBlock` as the sub-image
pub struct ImageBlock {
	pub x: u32,
	pub y: u32,
	pub block: PixlzrBlock,
}

#[derive(Clone, Debug)]
pub struct RawImage {
	pub alpha: bool,
	pub width: u32,
	pub height: u32,
	pub data: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct PixlzrBlockRaw {
	pub width: u32,
	pub height: u32,
	pub block_value: Option<f32>,
	pub data: RawImage,
}

#[cfg(feature = "image-rs")]
#[derive(Clone, Debug)]
pub struct PixlzrBlockImage {
	pub width: u32,
	pub height: u32,
	pub block_value: Option<f32>,
	pub data: DynamicImage,
}

#[derive(Clone, Debug)]
pub enum PixlzrBlock {
	Raw(PixlzrBlockRaw),
	#[cfg(feature = "image-rs")]
	Image(PixlzrBlockImage),
}

impl From<PixlzrBlockRaw> for PixlzrBlock {
	fn from(value: PixlzrBlockRaw) -> Self {
		PixlzrBlock::Raw(value)
	}
}
#[cfg(feature = "image-rs")]
impl From<PixlzrBlockImage> for PixlzrBlock {
	fn from(value: PixlzrBlockImage) -> Self {
		PixlzrBlock::Image(value)
	}
}

#[cfg(feature = "image-rs")]
impl From<PixlzrBlock> for PixlzrBlockImage {
	fn from(value: PixlzrBlock) -> Self {
		match value {
			PixlzrBlock::Image(image) => image,
			PixlzrBlock::Raw(raw) => {
				let (width, height, img) =
					(raw.width, raw.height, raw.data);
				let buf = img.data;
				let data: DynamicImage = if img.alpha {
					RgbaImage::from_raw(width, height, buf).unwrap().into()
				} else {
					RgbImage::from_raw(width, height, buf).unwrap().into()
				};
				Self {
					width,
					height,
					data,
					block_value: raw.block_value,
				}
			}
		}
	}
}

impl From<PixlzrBlock> for PixlzrBlockRaw {
	fn from(value: PixlzrBlock) -> Self {
		match value {
			PixlzrBlock::Raw(raw) => raw,
			#[cfg(feature = "image-rs")]
			PixlzrBlock::Image(image) => {
				let (width, height, img) =
					(image.width, image.height, image.data);
				let data = RawImage {
					width,
					height,
					alpha: img.as_rgba8().is_some(),
					data: img.into_bytes(),
				};
				Self {
					width,
					height,
					block_value: image.block_value,
					data,
				}
			}
		}
	}
}

impl PixlzrBlock {
	pub fn width(&self) -> u32 {
		match self {
			#[cfg(feature = "image-rs")]
			PixlzrBlock::Image(block) => block.width,
			PixlzrBlock::Raw(block) => block.width,
		}
	}
	pub fn height(&self) -> u32 {
		match self {
			#[cfg(feature = "image-rs")]
			PixlzrBlock::Image(block) => block.height,
			PixlzrBlock::Raw(block) => block.height,
		}
	}
	pub fn dimensions(&self) -> (u32, u32) {
		(self.width(), self.height())
	}
	pub fn block_value(&self) -> Option<f32> {
		match self {
			#[cfg(feature = "image-rs")]
			PixlzrBlock::Image(block) => block.block_value,
			PixlzrBlock::Raw(block) => block.block_value,
		}
	}
	pub fn has_alpha(&self) -> bool {
		match self {
			PixlzrBlock::Raw(raw) => raw.data.alpha,
			#[cfg(feature = "image-rs")]
			PixlzrBlock::Image(img) => img.data.color().has_alpha(),
		}
	}
	pub fn block_value_was_calculated(&self) -> bool {
		self.block_value().is_some()
	}
	pub fn as_slice(&self) -> &[u8] {
		match self {
			#[cfg(feature = "image-rs")]
			PixlzrBlock::Image(image) => image.data.as_bytes(),
			PixlzrBlock::Raw(raw) => raw.data.data.as_slice(),
		}
	}
	pub fn set_block_value(&mut self, value: f32) {
		match self {
			#[cfg(feature = "image-rs")]
			PixlzrBlock::Image(image) => image.block_value = Some(value),
			PixlzrBlock::Raw(raw) => raw.block_value = Some(value),
		}
	}
}

#[allow(clippy::match_wildcard_for_single_variants)]
impl PixlzrBlock {
	pub fn as_image(&self) -> Option<&PixlzrBlockImage> {
		match self {
			#[cfg(feature = "image-rs")]
			PixlzrBlock::Image(image) => Some(image),
			_ => None,
		}
	}
	pub fn as_raw(&self) -> Option<&PixlzrBlockRaw> {
		match self {
			PixlzrBlock::Raw(raw) => Some(raw),
			_ => None,
		}
	}
	pub fn is_image(&self) -> bool {
		match self {
			#[cfg(feature = "image-rs")]
			PixlzrBlock::Image(_) => true,
			_ => false,
		}
	}
	pub fn is_raw(&self) -> bool {
		matches!(self, PixlzrBlock::Raw(_))
	}
}

impl PixlzrBlock {
	pub fn pixels(&self) -> ChunksExact<u8> {
		let chunk_size = 3 + self.has_alpha() as usize;
		match self {
			#[cfg(feature = "image-rs")]
			PixlzrBlock::Image(image) => {
				image.data.as_bytes().chunks_exact(chunk_size)
			}
			PixlzrBlock::Raw(raw) => {
				raw.data.data.chunks_exact(chunk_size)
			}
		}
	}

	pub fn resize(
		&self,
		width: u32,
		height: u32,
		filter: I_FilterType,
	) -> Self {
		if self.dimensions() == (width, height) {
			return self.clone();
		}
		#[cfg(feature = "image-rs")]
		#[cfg(not(feature = "fir"))]
		{
			let mut img = PixlzrBlockImage::from(self);
			img.width = width;
			img.height = height;
			img.data = img.data.resize_exact(width, height, filter);
			return img.into();
		}

		use fast_image_resize::{images::Image, ResizeOptions};

		let alpha = self.has_alpha();
		let pixel_type = if alpha {
			FIR_PixelType::U8x4
		} else {
			FIR_PixelType::U8x3
		};

		let mut dst_image = Image::new(width, height, pixel_type);

		let resize_alg = filter_type_to_fir_resizing_alg(
			filter,
			width > self.width() || height > self.height(),
			2,
		);

		let mut resizer = Resizer::new();
		let mut bytes = self.as_slice().to_owned();
		resizer
			.resize(
				&Image::from_slice_u8(
					self.width(),
					self.height(),
					&mut bytes,
					pixel_type,
				)
				.unwrap(),
				&mut dst_image,
				&ResizeOptions::new().resize_alg(resize_alg),
			)
			.unwrap();

		PixlzrBlockRaw {
			width,
			height,
			block_value: None,
			data: RawImage {
				alpha,
				width,
				height,
				data: dst_image.into_vec(),
			},
		}
		.into()
	}
}

#[cfg(feature = "fir")]
fn filter_type_to_fir_resizing_alg(
	filter: I_FilterType,
	upscale: bool,
	multiplicity: u8,
) -> ResizeAlg {
	match filter {
		I_FilterType::Nearest => ResizeAlg::Nearest,
		f if upscale => match f {
			I_FilterType::Triangle => ResizeAlg::SuperSampling(
				FIR_FilterType::Bilinear,
				multiplicity,
			),
			I_FilterType::Lanczos3 => ResizeAlg::SuperSampling(
				FIR_FilterType::Lanczos3,
				multiplicity,
			),
			I_FilterType::Gaussian => ResizeAlg::SuperSampling(
				FIR_FilterType::Gaussian,
				multiplicity,
			),
			I_FilterType::CatmullRom => ResizeAlg::SuperSampling(
				FIR_FilterType::CatmullRom,
				multiplicity,
			),
			_ => unreachable!(),
		},
		f => match f {
			I_FilterType::Triangle => {
				ResizeAlg::Convolution(FIR_FilterType::Hamming)
			}
			I_FilterType::Lanczos3 => {
				ResizeAlg::Convolution(FIR_FilterType::Lanczos3)
			}
			I_FilterType::Gaussian => {
				ResizeAlg::Convolution(FIR_FilterType::Gaussian)
			}
			I_FilterType::CatmullRom => {
				ResizeAlg::Convolution(FIR_FilterType::CatmullRom)
			}
			_ => unreachable!(),
		},
	}
}
