#![allow(
	clippy::cast_precision_loss,
	clippy::cast_possible_truncation,
	clippy::cast_lossless,
	clippy::cast_sign_loss
)]

/// image_resize:
/// - image-rs, fir:
///   (img: &DynamicImage, ...) -> DynamicImage
/// - image-rs, !fir:
///   (img: &DynamicImage, ...) -> DynamicImage
/// - !image-rs, fir:
///   (img: &PixlzrBlock, ...) -> PixlzrBlock
/// - !image-rs, !fir:
///   panic!
use core::ops::{AddAssign, Mul};

use crate::data_types::PixlzrBlock;

#[cfg(feature = "image-rs")]
use image::{imageops::FilterType, GenericImageView, Pixel, Primitive};
use palette::{IntoColor, Oklab, Oklaba, Srgb, Srgba};

// #[cfg(not(feature = "image-rs"))]
/// TODO: Support for PixlzrBlock instead of DynamicImage
/// Calculates a `[0; 1]` value for the pixel variance of a given `img` image
///
/// 1. Calculates the average of pixel values
/// 2. Calculates the total difference of these values
/// 3. Normalizes the result to `[0; 1]`
pub fn get_block_variance<F0, F1>(
	block: &PixlzrBlock,
	before: &F0,
	after: &F1,
) -> f32
where
	F0: Fn(f32, f32) -> f32,
	F1: Fn(f32) -> f32,
{
	/*
	- $p_{i, j}$: pixel at position $i, j$
	- $W, H$: image's width and height
	- $M$: maximum subpixel value
	- $\bar p$: average pixel value
	- $\delta_{i, j}$: per pixel difference
	- $\int\delta$: sum of differences

	The maximum value for $\in\delta$ is when:
	- $p_{i, j} = M$ for half of the values of $i, j$,
	- and $0$ for the other half.
	So $\bar p = {M\over 2}$ and $\delta_{i, j} = {M\over 2}$.
	Thus, $\int\delta = W\cdot H\times \delta_{i, j} = {W\cdot H\cdot M\over 2}$.
	cont := W * H
	*/
	// 1. Calculates the average of pixel values
	let count = (block.width() * block.height()) as f32;
	if block.has_alpha() {
		let (average, count) = {
			let mut sum = [0.; 4];
			for pixel in block.pixels() {
				let color: Oklaba<f32> =
					Srgba::new(pixel[0], pixel[1], pixel[2], pixel[3])
						.into_linear()
						.into_color();
				sum[0] += color.a;
				sum[1] += color.b;
				sum[2] += color.l;
				sum[3] += color.alpha;
			}
			sum[0] /= count;
			sum[1] /= count;
			sum[2] /= count;
			sum[3] /= count;
			(sum, count)
		};

		// 2. Calculates the total difference between these values
		let delta = {
			let mut delta = [0.; 4];
			for pixel in block.pixels() {
				let color: Oklaba<f32> =
					Srgba::new(pixel[0], pixel[1], pixel[2], pixel[3])
						.into_linear()
						.into_color();
				delta[0] += before(color.a, average[0]);
				delta[1] += before(color.b, average[1]);
				delta[2] += before(color.l, average[2]);
				delta[3] += before(color.alpha, average[3]);
			}
			delta
		};
		// 3. Normalizes the result to `[0; 1]`
		let factor = count;
		after((delta[0] + delta[1] + delta[2] + delta[3]) / factor)
	} else {
		let (average, count) = {
			let mut sum = [0.; 3];
			for pixel in block.pixels() {
				let color: Oklab<f32> =
					Srgb::new(pixel[0], pixel[1], pixel[2])
						.into_linear()
						.into_color();
				sum[0] += color.a;
				sum[1] += color.b;
				sum[2] += color.l;
			}
			sum[0] /= count;
			sum[1] /= count;
			sum[2] /= count;
			(sum, count)
		};

		// 2. Calculates the total difference between these values
		let delta = {
			let mut delta = [0.; 3];
			for pixel in block.pixels() {
				let color: Oklab<f32> =
					Srgb::new(pixel[0], pixel[1], pixel[2])
						.into_linear()
						.into_color();
				delta[0] += before(color.a, average[0]);
				delta[1] += before(color.b, average[1]);
				delta[2] += before(color.l, average[2]);
			}
			delta
		};
		// 3. Normalizes the result to `[0; 1]`
		let factor = count;
		after((delta[0] + delta[1] + delta[2]) / factor)
	}
}

fn parse_value(value: f32) -> f32 {
	if value.is_sign_positive() {
		return value;
	}
	let value = (1f32 + value).max(0f32);
	if value.is_sign_positive() {
		value
	} else {
		1f32
	}
}

pub fn reduce_image_section(
	value: (f32, f32),
	block: &PixlzrBlock,
	filter_downscale: FilterType,
) -> PixlzrBlock {
	let value = (parse_value(value.0), parse_value(value.1));
	// println!("Post-value: {}", value.0);
	let level_hz = value.0.log2().round().min(0f32).exp2();
	let level_vr = value.1.log2().round().min(0f32).exp2();
	let (width, height) = block.dimensions();
	let width = (width as f64 * level_hz as f64).max(1f64).ceil() as u32;
	let height = (height as f64 * level_vr as f64).max(1f64).ceil() as u32;
	// Resizes the image down
	let mut img = block.resize(width, height, filter_downscale);
	img.set_block_value(value.0.hypot(value.1));
	img
}

#[inline]
#[allow(clippy::semicolon_if_nothing_returned)]
fn add_px<T, U>(acc: &mut [T], value: &Vec<U>, k: T)
where
	T: AddAssign<T> + From<U> + Mul<T, Output = T> + Copy,
	U: Copy,
{
	acc.iter_mut()
		.zip(value)
		.for_each(|(el, v)| *el += T::from(*v) * k)
}

const BASE_FACTOR: u64 = (2 << 11) as u64;

#[cfg(feature = "image-rs")]
fn pixel_channels<T, U, V>(img: &T, x: u32, y: u32) -> Vec<V>
where
	T: GenericImageView<Pixel = U>,
	U: Pixel<Subpixel = V>,
	V: Primitive + Copy,
{
	img.get_pixel(x, y).channels().to_owned()
}

#[cfg(feature = "image-rs")]
#[allow(clippy::identity_op)]
/// Calculates a `[0; 1]` value for the pixel variance of a given `img` image
///
/// 1. Calculates the average of pixel values
/// 2. Calculates the total difference of these values
/// 3. Normalizes the result to `[0; 1]`
pub fn get_block_variance_by_directions<T, U>(img: &T) -> (f32, f32)
where
	T: GenericImageView<Pixel = U>,
	U: Pixel<Subpixel = u8>,
{
	let abs = |x: &i16| x.unsigned_abs();
	// 1. Calculates the average of pixel values
	let channels = U::CHANNEL_COUNT as usize;
	let mut sum_hz = vec![0u64; channels];
	let mut sum_vr = vec![0u64; channels];

	let (width, height) = img.dimensions();

	for y in 0..height - 2 {
		for x in 0..width - 2 {
			let mut px_hz: Vec<i16> = vec![0i16; channels];
			let mut px_vr: Vec<i16> = vec![0i16; channels];

			let v00 = &pixel_channels(img, x + 0, y + 0);
			let v01 = &pixel_channels(img, x + 0, y + 1);
			let v02 = &pixel_channels(img, x + 0, y + 2);
			let v10 = &pixel_channels(img, x + 1, y + 0);
			let v12 = &pixel_channels(img, x + 1, y + 2);
			let v20 = &pixel_channels(img, x + 2, y + 0);
			let v21 = &pixel_channels(img, x + 2, y + 1);
			let v22 = &pixel_channels(img, x + 2, y + 2);

			// Horizontal
			add_px(&mut px_hz, v00, -1);
			add_px(&mut px_hz, v01, -2);
			add_px(&mut px_hz, v02, -1);

			add_px(&mut px_hz, v20, 1);
			add_px(&mut px_hz, v21, 2);
			add_px(&mut px_hz, v22, 1);

			// Vertical
			add_px(&mut px_vr, v00, -1);
			add_px(&mut px_vr, v10, -2);
			add_px(&mut px_vr, v20, -1);

			add_px(&mut px_vr, v02, 1);
			add_px(&mut px_vr, v12, 2);
			add_px(&mut px_vr, v22, 1);

			add_px(&mut sum_hz, &px_hz.iter().map(abs).collect(), 1);
			add_px(&mut sum_vr, &px_vr.iter().map(abs).collect(), 1);
		}
	}

	// 3. Normalizes the result to `[0; 1]`
	let factor =
		((width - 2) as u64 * (height - 2) as u64 * BASE_FACTOR) as f64;
	(
		(sum_hz.iter().sum::<u64>() as f64 / factor) as f32,
		(sum_vr.iter().sum::<u64>() as f64 / factor) as f32,
	)
}
