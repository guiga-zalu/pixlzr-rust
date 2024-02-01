use crate::data_types::{ImageBlock, PixlzrBlock, PixlzrBlockImage};

#[cfg(feature = "image-rs")]
use image::{DynamicImage, GenericImageView};

#[cfg(feature = "image-rs")]
/// Return a cut-out of this image delimited by the bounding rectangle.
///
/// Tests if `width` and `height` produce a valid rectangle inside the `img` image and, if not, alter them.
pub fn get_image_block<'a>(
	image: &'a DynamicImage,
	x: u32,
	y: u32,
	mut width: u32,
	mut height: u32,
) -> PixlzrBlock {
	let (iw, ih) = image.dimensions();
	width = if x + width > iw { iw - x } else { width };
	height = if y + height > ih { ih - y } else { height };
	PixlzrBlockImage {
		width,
		height,
		block_value: None,
		data: image.crop_imm(x, y, width, height),
	}
	.into()
}

#[cfg(feature = "image-rs")]
/// Splits the `img` image into blocks of size up to `width` x `height`, and returns the blocks as a `Vec<SubImagem>`
pub fn split_image<'a>(
	image: &'a DynamicImage,
	width: u32,
	height: u32,
) -> Vec<ImageBlock> {
	let (iw, ih) = image.dimensions();
	let mut seccoes: Vec<ImageBlock> = vec![];
	// Block's width and height
	let bw = (iw as f32 / width as f32).ceil() as u32;
	let bh = (ih as f32 / height as f32).ceil() as u32;
	// For each block
	for mut y in 0..bh {
		y *= height;
		for mut x in 0..bw {
			// Get's the block
			x *= width;
			let block = get_image_block(image, x, y, width, height);
			let sub = ImageBlock { block, x, y };
			// Save the block
			seccoes.push(sub);
		}
	}
	// Return blocks
	seccoes
}
