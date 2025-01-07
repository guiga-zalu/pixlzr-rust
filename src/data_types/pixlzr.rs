use super::{block::*, FilterType};

use crate::operations::{
	get_block_variance, get_block_variance_directionally,
	reduce_image_section,
};

use std::iter::Iterator;

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
	/// Returns the width and height of the image as a tuple of u32.
	pub fn dimensions(&self) -> (u32, u32) {
		(self.width, self.height)
	}
	/// Returns the common block dimensions in the image as a tuple of (width, height).
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
	/// Returns a tuple of two booleans indicating if there are trailing blocks
	/// in both the horizontal and vertical directions of the block grid.
	///
	/// A trailing block is a block that is below the full size of (`block_width`, `block_height`).
	pub fn block_grid_has_trailing(&self) -> (bool, bool) {
		(
			self.width % self.block_width > 0,
			self.height % self.block_height > 0,
		)
	}

	/// Returns a parallel iterator over the image's blocks organized in lines, with the amount of lines equal to the vertical size of the block grid.
	///
	/// Each element of the iterator is a slice of blocks, with the length equal to the horizontal size of the block grid.
	///
	/// Like `lines`, but parallel through `rayon`.
	pub fn par_lines(&self) -> rayon::slice::ChunksExact<PixlzrBlock> {
		self.blocks
			.par_chunks_exact(self.block_grid_width() as usize)
	}

	/// Returns an iterator over the image's blocks organized in lines, with the amount of lines equal to the vertical size of the block grid.
	///
	/// Each element of the iterator is a slice of blocks, with the length equal to the horizontal size of the block grid.
	///
	/// Like `par_lines`, but not parallel.
	pub fn lines(&self) -> std::slice::ChunksExact<PixlzrBlock> {
		self.blocks.chunks_exact(self.block_grid_width() as usize)
	}

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
						block.resize(nwidth, nheight, ifilter)
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

	pub fn shrink(
		&mut self,
		filter_downscale: FilterType,
		before_average: &fn(f32, f32) -> f32,
		after_average: &fn(f32) -> f32,
	) {
		let filter_downscale = filter_downscale.into();
		self.blocks = self
			.blocks
			.iter()
			.map({
				|block| {
					if block.block_value().is_some() {
						return (*block).clone();
					}
					// Calculate the value
					let value = get_block_variance(
						&block,
						&before_average,
						&after_average,
					);
					reduce_image_section(
						(value, value),
						block,
						filter_downscale,
					)
				}
			})
			.collect();
	}

	#[inline]
	pub fn shrink_by(
		&mut self,
		filter_downscale: FilterType,
		factor: f32,
	) {
		let before_average: fn(f32, f32) -> f32 =
			|x: f32, avg: f32| (x - avg).abs();
		let after_average = |x: f32| x * factor * BASE_FACTOR;
		let filter_downscale = filter_downscale.into();
		self.blocks = self
			.blocks
			.iter()
			.map({
				|block| {
					if block.block_value().is_some() {
						return (*block).clone();
					}
					// Calculate the value
					let value = get_block_variance(
						&block,
						&before_average,
						&after_average,
					);
					reduce_image_section(
						(value, value),
						block,
						filter_downscale,
					)
				}
			})
			.collect();
	}

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
				// Calculate the value
				let value = get_block_variance_directionally(block);
				reduce_image_section(
					(value.0 * factor, value.1 * factor),
					block,
					filter_downscale,
				)
			})
			.collect();
	}
}
