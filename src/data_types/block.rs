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
pub struct PixlzrBlockRaw {
	pub width: u32,
	pub height: u32,
	pub block_value: Option<f32>,
	pub data: Vec<u8>,
	pub alpha: bool,
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
				let (width, height) = (raw.width, raw.height);
				let data_vec = raw.data.to_vec();
				let data: DynamicImage = if raw.alpha {
					RgbaImage::from_raw(width, height, data_vec)
						.unwrap()
						.into()
				} else {
					RgbImage::from_raw(width, height, data_vec)
						.unwrap()
						.into()
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

impl PixlzrBlock {
	pub fn width(&self) -> u32 {
		match self {
			#[cfg(feature = "image-rs")]
			PixlzrBlock::Image(block) => (*block).width,
			PixlzrBlock::Raw(block) => (*block).width,
		}
	}
	pub fn height(&self) -> u32 {
		match self {
			#[cfg(feature = "image-rs")]
			PixlzrBlock::Image(block) => (*block).height,
			PixlzrBlock::Raw(block) => (*block).height,
		}
	}
	pub fn dimensions(&self) -> (u32, u32) {
		(self.width(), self.height())
	}
	pub fn block_value(&self) -> Option<f32> {
		match self {
			#[cfg(feature = "image-rs")]
			PixlzrBlock::Image(block) => (*block).block_value,
			PixlzrBlock::Raw(block) => (*block).block_value,
		}
	}
	pub fn has_alpha(&self) -> bool {
		match self {
			PixlzrBlock::Raw(raw) => raw.alpha,
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
			PixlzrBlock::Raw(raw) => raw.data.as_slice(),
		}
	}
}

impl PixlzrBlock {
	#[cfg(feature = "image-rs")]
	pub fn as_image(&self) -> Option<&PixlzrBlockImage> {
		match self {
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
		match self {
			PixlzrBlock::Raw(_) => true,
			_ => false,
		}
	}
}
