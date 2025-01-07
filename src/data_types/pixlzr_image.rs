use super::{block::*, iter::*, pixlzr::Pixlzr, FilterType};

use image::{DynamicImage, GenericImage};

impl Pixlzr {
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
				let img = PixlzrBlockImage::from(block.clone()).data;
				output
					.copy_from(
						&img,
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
