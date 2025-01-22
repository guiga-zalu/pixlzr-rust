// TODO: Conferir via clippy
#![allow(clippy::all, clippy::must_use_candidate, clippy::cast_sign_loss)]
use crate::{
	data_types::{block::PixlzrBlockImage, FilterType as P_FilterType},
	operations::*,
};

use crate::split::split_image;

use image::{DynamicImage, GenericImage};

macro_rules! dpl {
	($T:ty) => {
		($T, $T)
	};
}

///
///
///
///
/// If threshold is negative, invert the operation
pub fn process_custom(
	image: &DynamicImage,
	threshold: f32,
	block_size: dpl!(u32),
	min_block_size: dpl!(u32),
	filters: dpl!(P_FilterType),
	before_average: &fn(f32, f32) -> f32,
	after_average: &fn(f32) -> f32,
) -> DynamicImage {
	let (block_width, block_height) = block_size;
	let min_block_width = min_block_size.0.max(4);
	let min_block_height = min_block_size.1.max(4);
	if block_width <= min_block_width || block_height <= min_block_height {
		return image.clone();
	}
	let is_positive = threshold >= 0.0;
	let threshold = threshold.abs();

	let (filter_downscale, filter_upscale) = filters;

	// New image
	let mut output =
		DynamicImage::new_rgba8(image.width(), image.height());
	// For each splitten block
	for section in split_image(image, block_width, block_height) {
		// Get the block and it's dimensions
		let (x, y) = (section.x, section.y);
		let block = section.block;
		let (w0, h0) = block.dimensions();
		// Calculate the value
		let value =
			get_block_variance(&block, &before_average, &after_average);
		// Post-process the value
		let img = if (value >= threshold) ^ is_positive {
			// Calculate the resize level and dimensions
			PixlzrBlockImage::from(
				reduce_image_section(
					(value, value),
					&block,
					filter_downscale,
				)
				.resize(w0, h0, filter_upscale),
			)
			.data
		} else {
			process_custom(
				&PixlzrBlockImage::from(block.clone()).data,
				threshold,
				(block_width >> 1, block_height >> 1),
				(min_block_width, min_block_height),
				(filter_downscale, filter_upscale),
				before_average,
				after_average,
			)
		};
		// Saves it's data in the output buffer
		output.copy_from(&img, x, y).unwrap();
	}
	// Returns the new image
	output
}

///
///
///
///
pub fn process(
	image: &DynamicImage,
	block_size: u32,
	threshold: f32,
) -> DynamicImage {
	let before_average: fn(f32, f32) -> f32 =
		|x: f32, avg: f32| (x - avg).abs();
	// |x, avg| (x - avg).pow(2)
	let after_average: fn(f32) -> f32 = |x: f32| x;
	// |x| x.sqrt()

	process_custom(
		image,
		threshold,
		(block_size, block_size),
		(4, 4),
		(P_FilterType::Lanczos3, P_FilterType::Nearest),
		&before_average,
		&after_average,
	)
}

// /
// /
// /
// /
// pub fn process_full<F0>(
//     image: &DynamicImage,
//     threshold: f32,
//     fn_variance: Option<F0>,
// ) -> DynamicImage
// where
//     F0: (Fn(f32) -> f32) + Copy,
// {
//     if let Some(_fn_variance) = fn_variance {
//         return process_custom(
//             image,
//             threshold,
//             image.width(),
//             image.height(),
//             4,
//             4,
//             FilterType::Lanczos3,
//             FilterType::Nearest,
//             _fn_variance,
//         );
//     }
//     let fn_variance = |x: f32| x;

//     process_custom(
//         image,
//         threshold,
//         image.width(),
//         image.height(),
//         4,
//         4,
//         FilterType::Lanczos3,
//         FilterType::Nearest,
//         fn_variance,
//     )
// }
