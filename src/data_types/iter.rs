#![cfg(feature = "image-rs")]

use super::PixlzrBlock;

use crate::split::get_image_block;

use image::{DynamicImage, GenericImageView};

/// Image block representation, with:
/// - `x: u32, y: u32` as the coordinates of the block
/// - `block: DynamicImage` as the sub-image
pub struct ImageBlockIterator<'a> {
	bwidth: u32,
	bheight: u32,
	image: &'a DynamicImage,
	horizontal_blocks: u32,
	vertical_blocks: u32,
	curr_x: u32,
	curr_y: u32,
}

impl<'a> ImageBlockIterator<'a> {
	#[allow(
		clippy::cast_sign_loss,
		clippy::cast_possible_truncation,
		clippy::cast_lossless
	)]
	pub fn new(
		image: &'a DynamicImage,
		bwidth: u32,
		bheight: u32,
	) -> Self {
		let (image_width, image_height) = image.dimensions();
		Self {
			bwidth,
			bheight,
			image,
			horizontal_blocks: (image_width as f64 / bwidth as f64).ceil()
				as u32,
			vertical_blocks: (image_height as f64 / bheight as f64).ceil()
				as u32,
			curr_x: 0,
			curr_y: 0,
		}
	}
	fn get_block(&self, x: u32, y: u32) -> PixlzrBlock {
		if x > self.horizontal_blocks || y > self.vertical_blocks {
			panic!("Pânico!");
		}
		let bwidth = self.bwidth;
		let bheight = self.bheight;
		get_image_block(
			self.image,
			x * bwidth,
			y * bheight,
			bwidth,
			bheight,
		)
	}
}

impl<'a> Iterator for ImageBlockIterator<'a> {
	type Item = PixlzrBlock;
	fn next(&mut self) -> Option<Self::Item> {
		if self.curr_x == self.horizontal_blocks {
			self.curr_x = 0;
			self.curr_y += 1;
		}
		if self.curr_y == self.vertical_blocks {
			None
		} else {
			let block = self.get_block(self.curr_x, self.curr_y);
			self.curr_x += 1;
			Some(block)
		}
	}
	fn size_hint(&self) -> (usize, Option<usize>) {
		let size = self.len();
		(size, Some(size))
	}
}

impl<'a> ExactSizeIterator for ImageBlockIterator<'a> {
	fn len(&self) -> usize {
		self.horizontal_blocks as usize * self.vertical_blocks as usize
	}
}
