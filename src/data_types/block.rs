#[cfg(feature = "image-rs")]
use image::{DynamicImage, RgbImage, RgbaImage};

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
