use crate::data_types::{ImageBlock, PixlzrBlock, PixlzrBlockImage};

#[cfg(feature = "image-rs")]
use image::{DynamicImage, GenericImageView};

#[cfg(feature = "image-rs")]
/// Return a cut-out of this image delimited by the bounding rectangle.
///
/// Tests if `width` and `height` produce a valid rectangle inside the `img` image and, if not, alter them.
pub fn get_image_block(
	image: &DynamicImage,
	x: u32,
	y: u32,
	mut width: u32,
	mut height: u32,
) -> PixlzrBlock {
	let (iw, ih) = image.dimensions();
	width = width.min(iw - x);
	height = height.min(ih - y);
	PixlzrBlockImage {
		width,
		height,
		block_value: None,
		data: image.crop_imm(x, y, width, height),
	}
	.into()
}

#[cfg(feature = "image-rs")]
#[allow(
	clippy::cast_lossless,
	clippy::cast_sign_loss,
	clippy::cast_possible_truncation,
	clippy::module_name_repetitions
)]
/// Splits the `img` image into blocks of size up to `width` x `height`, and returns the blocks as a `Vec<SubImagem>`
pub fn split_image(
	image: &DynamicImage,
	width: u32,
	height: u32,
) -> Vec<ImageBlock> {
	let (iw, ih) = image.dimensions();
	let mut seccoes: Vec<ImageBlock> = vec![];
	// Block's width and height
	let bw = (iw as f64 / width as f64).ceil() as u32;
	let bh = (ih as f64 / height as f64).ceil() as u32;
	// For each block
	for mut y in 0..bh {
		y *= height;
		for mut x in 0..bw {
			// Get's the block
			x *= width;
			let block = get_image_block(image, x, y, width, height);
			let sub = ImageBlock { x, y, block };
			// Save the block
			seccoes.push(sub);
		}
	}
	// Return blocks
	seccoes
}
