use image::{imageops::FilterType, DynamicImage, GenericImage, GenericImageView, Rgba};
use std::vec::Vec;

/// Return a cut-out of this image delimited by the bounding rectangle.
///
/// Tests if `width` and `height` produce a valid rectangle inside the `img` image and, if not, alter them.
fn get_image_block<'a>(
	image: &'a DynamicImage,
	x: u32,
	y: u32,
	mut width: u32,
	mut height: u32,
) -> DynamicImage {
	let (iw, ih) = image.dimensions();
	width = if x + width > iw { iw - x } else { width };
	height = if y + height > ih { ih - y } else { height };
	image.crop_imm(x, y, width, height)
}

/// Image block representation, with:
/// - `x: u32, y: u32` as the coordinates of the block
/// - `block: DynamicImage` as the sub-image
pub struct ImageBlock {
	pub x: u32,
	pub y: u32,
	pub block: DynamicImage,
}

/// Splits the `img` image into blocks of size up to `width` x `height`, and returns the blocks as a `Vec<SubImagem>`
pub fn split_image<'a>(image: &'a DynamicImage, width: u32, height: u32) -> Vec<ImageBlock> {
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

/// Calculates a `[0; 1]` value for the pixel variance of a given `img` image
///
/// 1. Calculates the average of pixel values
/// 2. Calculates the total difference of these values
///    - Uses `difference := sum of { |pixel - average| } for p in pixels`
/// 3. Normalizes the result to `[0; 1]`
pub fn get_block_variance<T>(img: &T) -> f32
where
	T: GenericImageView<Pixel = Rgba<u8>>,
{
	// 1. Calculates the average of pixel values
	let mut sum: [u32; 4] = [0, 0, 0, 0];
	let mut count: u64 = 0;
	for (_x, _y, pixel) in img.pixels() {
		let rgba = pixel.0;
		sum[0] += rgba[0] as u32;
		sum[1] += rgba[1] as u32;
		sum[2] += rgba[2] as u32;
		sum[3] += rgba[3] as u32;
		count += 1;
	}
	let count = count as f32;
	let average = [
		sum[0] as f32 / count,
		sum[1] as f32 / count,
		sum[2] as f32 / count,
		sum[3] as f32 / count,
	];
	// 2. Calculates the total difference of these values
	let mut delta: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
	for (_x, _y, pixel) in img.pixels() {
		let rgba = pixel.0;
		delta[0] += ((rgba[0] as f32) - average[0]).abs();
		delta[1] += ((rgba[1] as f32) - average[1]).abs();
		delta[2] += ((rgba[2] as f32) - average[2]).abs();
		delta[3] += ((rgba[3] as f32) - average[3]).abs();
	}
	// 3. Normalizes the result to `[0; 1]`
	/*
	- $p_{i, j}$: pixel at position $i, j$
	- $W, H$: image's width and height
	- $M$: maximum subpixel value
	- $\bar p$: average pixel value
	- $\delta_{i, j}$: per pixel difference
	- $\int\delta$: sum of differences
	The maximum value for $\in\delta$ is* when:
	- $p_{i, j} = M$ for half of the values of $i, j$,
	- and $0$ for the other half.
	So $\bar p = {M\over 2}$ and $\delta_{i, j} = {M\over 2}$.
	Thus, $\int\delta = W\cdot H\times \delta_{i, j} = {W\cdot H\cdot M\over 2}$.
	cont := W * H
	 */
	let factor = count * (u8::MAX / 2) as f32;
	(delta[0] + delta[1] + delta[2] + delta[3]) / factor
}

/* pub fn custom_processer<F0, F1>(
	block_width: u32,
	block_height: u32,
	filter_downscale: FilterType,
	filter_upscale: FilterType,
	fn_variance: F0,
) -> F1
where
	F0: Fn(f32) -> f32,
	F1: Fn(&DynamicImage) -> DynamicImage
{
	let process = |image: &DynamicImage| -> DynamicImage {
		// New image
		let mut output = DynamicImage::new_rgba8(image.width(), image.height());
		// For each splitten block
		for section in split_image(&image, block_width, block_height) {
			// Get the block and it's dimensions
			let block: DynamicImage = section.block;
			let (w0, h0) = (block.width(), block.height());
			// Calculate the value
			let value = get_block_variance(&block);
			// Post-process the value
			let value = fn_variance(value);
			// Calculate the resize level and dimensions
			let level = value.log2().round().min(0f32).exp2();
			let w1 = (w0 as f32 * level).ceil() as u32;
			let h1 = (h0 as f32 * level).ceil() as u32;
			// Resizes the image down and back
			let img = block
				.resize(w1, h1, filter_downscale)
				.resize(w0, h0, filter_upscale);
			// Saves it's data in the output buffer
			output.copy_from(&img, section.x, section.y).unwrap();
		}
		// Returns the new image
		output
	};
	process
} */

pub fn custom_process<F0>(
	image: &DynamicImage,
	block_width: u32,
	block_height: u32,
	filter_downscale: FilterType,
	filter_upscale: FilterType,
	fn_variance: F0,
) -> DynamicImage
where
	F0: Fn(f32) -> f32,
{
	// New image
	let mut output = DynamicImage::new_rgba8(image.width(), image.height());
	// For each splitten block
	for section in split_image(&image, block_width, block_height) {
		// Get the block and it's dimensions
		let block: DynamicImage = section.block;
		let (w0, h0) = (block.width(), block.height());
		// Calculate the value
		let value = get_block_variance(&block);
		// Post-process the value
		let value = fn_variance(value);
		// Calculate the resize level and dimensions
		let level = value.log2().round().min(0f32).exp2();
		let w1 = (w0 as f32 * level).ceil() as u32;
		let h1 = (h0 as f32 * level).ceil() as u32;
		// Resizes the image down and back
		let img = block
			.resize(w1, h1, filter_downscale)
			.resize(w0, h0, filter_upscale);
		// Saves it's data in the output buffer
		output.copy_from(&img, section.x, section.y).unwrap();
	}
	// Returns the new image
	output
}

pub fn process<F0>(image: &DynamicImage, block_size: u32, fn_variance: Option<F0>) -> DynamicImage
where
	F0: (Fn(f32) -> f32) + Copy,
{
	if let Some(fn_variance_value) = fn_variance {
		return custom_process(
			image,
			block_size,
			block_size,
			FilterType::Lanczos3,
			FilterType::Nearest,
			fn_variance_value,
		);
	}
	return custom_process(
		image,
		block_size,
		block_size,
		FilterType::Lanczos3,
		FilterType::Nearest,
		default_fn_variance,
	);
}

pub fn default_fn_variance(value: f32) -> f32 {
	value
}

pub fn custom_tree_process<F0>(
	image: &DynamicImage,
	threshold: f32,
	block_width: u32,
	block_height: u32,
	mut min_block_width: u32,
	mut min_block_height: u32,
	filter_downscale: FilterType,
	filter_upscale: FilterType,
	fn_variance: F0,
) -> DynamicImage
where
	F0: (Fn(f32) -> f32) + Copy,
{
	min_block_width = min_block_width.max(4);
	min_block_height = min_block_height.max(4);
	if block_width <= min_block_width || block_height <= min_block_height {
		return image.clone();
	}
	// New image
	let mut output = DynamicImage::new_rgba8(image.width(), image.height());
	// For each splitten block
	for section in split_image(&image, block_width, block_height) {
		// Get the block and it's dimensions
		let block: DynamicImage = section.block;
		let (x, y) = (section.x, section.y);
		let (w0, h0) = (block.width(), block.height());
		// Calculate the value
		let value = get_block_variance(&block);
		// Post-process the value
		let value = fn_variance(value);
		if value >= threshold {
			let img = custom_tree_process(
				&block,
				threshold,
				block_width >> 1,
				block_height >> 1,
				min_block_width,
				min_block_height,
				filter_downscale,
				filter_upscale,
				fn_variance,
			);
			output.copy_from(&img, x, y).unwrap();
		} else {
			// Calculate the resize level and dimensions
			let level = value.log2().round().min(0f32).exp2();
			let w1 = (w0 as f32 * level).ceil() as u32;
			let h1 = (h0 as f32 * level).ceil() as u32;
			// Resizes the image down and back
			let img = block
				.resize(w1, h1, filter_downscale)
				.resize(w0, h0, filter_upscale);
			// Saves it's data in the output buffer
			output.copy_from(&img, x, y).unwrap();
		}
	}
	// Returns the new image
	output
}

pub fn tree_process<F0>(
	image: &DynamicImage,
	block_size: u32,
	threshold: f32,
	fn_variance: Option<F0>,
) -> DynamicImage
where
	F0: (Fn(f32) -> f32) + Copy,
{
	if let Some(fn_variance_value) = fn_variance {
		return custom_tree_process(
			image,
			threshold,
			block_size,
			block_size,
			4,
			4,
			FilterType::Lanczos3,
			FilterType::Nearest,
			fn_variance_value,
		);
	}
	return custom_tree_process(
		image,
		threshold,
		block_size,
		block_size,
		4,
		4,
		FilterType::Lanczos3,
		FilterType::Nearest,
		default_fn_variance,
	);
}

pub fn tree_full_process<F0>(
	image: &DynamicImage,
	threshold: f32,
	fn_variance: Option<F0>,
) -> DynamicImage
where
	F0: (Fn(f32) -> f32) + Copy,
{
	if let Some(fn_variance_value) = fn_variance {
		return custom_tree_process(
			image,
			threshold,
			image.width(),
			image.height(),
			4,
			4,
			FilterType::Lanczos3,
			FilterType::Nearest,
			fn_variance_value,
		);
	}
	return custom_tree_process(
		image,
		threshold,
		image.width(),
		image.height(),
		4,
		4,
		FilterType::Lanczos3,
		FilterType::Nearest,
		default_fn_variance,
	);
}
