use super::{block::*, FilterType};

#[cfg(feature = "image-rs")]
use super::iter::*;

use crate::operations::{
	get_block_variance, get_block_variance_by_directions,
	reduce_image_section,
};

use std::iter::Iterator;

#[cfg(feature = "image-rs")]
use image::{DynamicImage, GenericImage, GenericImageView};

use rayon::{
	iter::{IndexedParallelIterator, ParallelIterator},
	slice::ParallelSlice,
};

const BASE_FACTOR: f32 = 10.0;

#[derive(Clone)]
pub struct Pixlzr {
	pub width: u32,
	pub height: u32,
	pub block_width: u32,
	pub block_height: u32,
	pub filter: Option<FilterType>,
	pub blocks: Vec<PixlzrBlock>,
}

impl Pixlzr {
	pub fn dimensions(&self) -> (u32, u32) {
		(self.width, self.height)
	}
	pub fn block_dimensions(&self) -> (u32, u32) {
		(self.block_width, self.block_height)
	}
	#[inline]
	pub fn block_grid_width(&self) -> u32 {
		(self.width as f32 / self.block_width as f32).ceil() as u32
	}
	#[inline]
	pub fn block_grid_height(&self) -> u32 {
		(self.height as f32 / self.block_height as f32).ceil() as u32
	}
	pub fn block_grid_dimensions(&self) -> (u32, u32) {
		(self.block_grid_width(), self.block_grid_height())
	}
	pub fn block_grid_has_trailing(&self) -> (bool, bool) {
		(
			self.width % self.block_width > 0,
			self.height % self.block_height > 0,
		)
	}
	#[cfg(feature = "image-rs")]
	pub fn from_image(
		image: &DynamicImage,
		block_width: u32,
		block_height: u32,
	) -> Pixlzr {
		let blocks: Vec<_> =
			ImageBlockIterator::new(image, block_width, block_height)
				.collect();
		Self {
			width: image.width(),
			height: image.height(),
			block_width,
			block_height,
			filter: None,
			blocks,
		}
	}

	pub fn par_lines<'a>(
		&'a self,
	) -> rayon::slice::ChunksExact<'a, PixlzrBlock> {
		self.blocks
			.par_chunks_exact(self.block_grid_width() as usize)
	}

	pub fn lines<'a>(
		&'a self,
	) -> std::slice::ChunksExact<'a, PixlzrBlock> {
		self.blocks.chunks_exact(self.block_grid_width() as usize)
	}
}

impl Pixlzr {
	#[cfg(feature = "image-rs")]
	pub fn expand(&self, filter: FilterType) -> Self {
		let ifilter = filter.into();
		// Extract properties
		let (width, height) = self.dimensions();
		let (block_width, block_height) = self.block_dimensions();
		let (cols, rows) = self.block_grid_dimensions();

		let trailing_horizontal = width % block_width;
		let trailing_vertical = height % block_height;
		let has_trailing = self.block_grid_has_trailing();

		// Create list of blocks to be returned
		let blocks: Vec<PixlzrBlock> = self
			.par_lines()
			.zip(0..rows)
			.flat_map(|(line, y)| {
				let nheight = if y == rows - 1 && has_trailing.1 {
					trailing_vertical
				} else {
					block_height
				};

				// For each block
				line.iter()
					.zip(0..cols)
					.map(|(block, x)| {
						// Reduce the cumulatime limit
						let nwidth = if x == cols - 1 && has_trailing.0 {
							trailing_horizontal
						} else {
							block_width
						};
						// Extract it's image
						let img =
							PixlzrBlockImage::from(block.clone()).data;
						let pix_img = PixlzrBlockImage {
							width: nwidth,
							height: nheight,
							data: if img.dimensions() == (nwidth, nheight)
							{
								img
							} else {
								img.resize_exact(nwidth, nheight, ifilter)
							},
							block_value: block.block_value(),
						};
						pix_img.into()
					})
					.collect::<Vec<PixlzrBlock>>()
			})
			.collect();

		Self {
			width: self.width,
			height: self.height,
			block_width,
			block_height,
			filter: Some(filter),
			blocks,
		}
	}

	#[cfg(feature = "image-rs")]
	pub fn shrink<F0, F1>(
		&mut self,
		filter_downscale: FilterType,
		before_average: F0,
		after_average: F1,
	) where
		F0: Clone + Fn(f32, f32) -> f32,
		F1: Clone + Fn(f32) -> f32,
	{
		let filter_downscale = filter_downscale.into();
		self.blocks = self
			.blocks
			.iter()
			.map({
				let before_average = before_average.clone();
				let after_average = after_average.clone();
				move |block| {
					if block.block_value().is_some() {
						return (*block).clone();
					}
					let block = block.as_image().unwrap();
					let img = &block.data;
					// Calculate the value
					let value = get_block_variance(
						img,
						&before_average,
						&after_average,
					);
					let reduced = reduce_image_section(
						(value, value),
						&img,
						filter_downscale,
					);
					debug_assert_eq!(reduced.width, reduced.data.width());
					debug_assert_eq!(
						reduced.height,
						reduced.data.height()
					);
					reduced.into()
				}
			})
			.collect();
	}

	#[inline]
	#[cfg(feature = "image-rs")]
	pub fn shrink_by(
		&mut self,
		filter_downscale: FilterType,
		factor: f32,
	) {
		let before_average = |x: f32, avg: f32| (x - avg).abs();
		let after_average = |x: f32| x * factor * BASE_FACTOR;
		self.shrink(filter_downscale, before_average, after_average);
	}

	#[cfg(feature = "image-rs")]
	pub fn shrink_directionally(
		&mut self,
		filter_downscale: FilterType,
		factor: f32,
	) {
		let filter_downscale = filter_downscale.into();
		self.blocks = self
			.blocks
			.iter()
			.map(|block| {
				let block = block.as_image().unwrap();
				let img = &block.data;
				// Calculate the value
				let value = get_block_variance_by_directions(img);
				let reduced = reduce_image_section(
					(value.0 * factor, value.1 * factor),
					&img,
					filter_downscale,
				);
				debug_assert_eq!(reduced.width, reduced.data.width());
				debug_assert_eq!(reduced.height, reduced.data.height());
				reduced.into()
			})
			.collect();
	}

	#[cfg(feature = "image-rs")]
	pub fn to_image(&self, filter: FilterType) -> DynamicImage {
		// println!("Pre-expansion");
		let pix = self.expand(filter);
		// println!("Post-expansion");
		let mut output =
			if pix.blocks.iter().any(|block| block.has_alpha()) {
				DynamicImage::new_rgba8(self.width, self.height)
			} else {
				DynamicImage::new_rgb8(self.width, self.height)
			};
		let (block_width, block_height) = pix.block_dimensions();
		// let cols = (self.width as f32 / block_width as f32).ceil() as u32;
		// let mut x = 0;
		// let mut y = 0;
		// println!(
		// 	"b wh: {:?}, out wh: {:?}",
		// 	pix.block_dimensions(),
		// 	pix.dimensions()
		// );
		pix.lines().enumerate().for_each(|(y, line)| {
			line.iter().enumerate().for_each(|(x, block)| {
				let img = &block.as_image().unwrap().data;
				output
					.copy_from(
						img,
						x as u32 * block_width,
						y as u32 * block_height,
					)
					.unwrap();
			})
		});
		// for block in pix.blocks {
		// 	let img = &block.as_image().unwrap().data;
		// 	// println!(
		// 	// 	"xy: ({x}, {y}),\t{:?} => ({}, {})\tim wh: {:?}",
		// 	// 	(x * block_width, y * block_height),
		// 	// 	x * block_width + img.width(),
		// 	// 	y * block_height + img.height(),
		// 	// 	img.dimensions(),
		// 	// );
		// 	output
		// 		.copy_from(img, x * block_width, y * block_height)
		// 		.unwrap();
		// 	x += 1;
		// 	if x == cols {
		// 		x = 0;
		// 		y += 1;
		// 	}
		// }
		output
	}
}

impl From<Pixlzr> for DynamicImage {
	fn from(value: Pixlzr) -> Self {
		value.to_image(value.filter.unwrap_or(FilterType::Gaussian))
	}
}
