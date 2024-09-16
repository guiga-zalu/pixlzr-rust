#[macro_use]
pub mod bytes;

use self::bytes::Bytes;
use crate::{
	constants::*,
	data_types::{Pixlzr, PixlzrBlock, PixlzrBlockImage, Semver},
};

use phf;
use qoi::{self, Result as Result_QOI};

#[cfg(feature = "image-rs")]
use image::{RgbImage, RgbaImage};

#[allow(unused_imports)]
use rayon::iter::{IntoParallelIterator, ParallelIterator};

static VERSION_FILTER: phf::Map<&'static str, &'static Semver> = phf::phf_map! {
	"filter"    => &Semver::new(0, 0, 1),
	"line-sizes"=> &Semver::new(0, 0, 2),
};

fn has_resource(version: &Semver, resource_name: &str) -> bool {
	VERSION_FILTER
		.get(resource_name)
		.map_or(false, |resource_version| version >= *resource_version)
}

type Raw = Vec<u8>;

const ERR_ENDED_TOO_SOON: &str = "The slice ended to soon";

impl Pixlzr {
	/// Encodes the PIXLZR image into a vector of bytes, following the steps:
	/// 1. Prepares the image
	/// 2. Encodes the header
	/// 3. rayon: Gets each line of blocks
	///    - Encode each block
	///    - Get the encoded length
	/// 4. Gets and writes the length of each line
	/// 5. Appends each block to the final array
	pub fn encode_to_vec(&self) -> Result_QOI<Raw> {
		// Gets the numbers of columns and rows
		let (cols, rows) = (
			self.block_grid_width() as usize,
			self.block_grid_height() as usize,
		);

		let mut image =
			Bytes::new(vec![0; PIXLZR_HEADER_SIZE + (rows * 4)]);

		// Start encoding with the header
		image.write_slice(PIXLZR_MAGIC_NUMBERS);
		image.write_slice(PIXLZR_MAGIC_VERSION);
		image.write_u8(self.filter.unwrap_or_default() as u8);
		image.write_u32(self.width);
		image.write_u32(self.height);
		image.write_u32(self.block_width);
		image.write_u32(self.block_height);

		let (mut blocks, block_lengths): (Vec<Raw>, Vec<usize>) = {
			self.par_lines()
				.flat_map(|line| {
					// For each line of blocks
					line.iter()
						.map(|block| {
							// For each block
							// Encode the block, and get the encoded length
							let (output, len) = encode_block(block);
							(output.data, len)
						})
						.collect::<Vec<(Raw, usize)>>()
				})
				.collect::<Vec<(Raw, usize)>>()
				.into_iter()
				.unzip()
		};

		// For each line, write its size
		(0..rows).for_each(|row| {
			let idx = row * cols;
			let sum: usize = block_lengths[idx..(idx + cols)].iter().sum();
			image.write_u32(sum as u32);
		});

		// Append the blocks
		for block in blocks.iter_mut() {
			image.write_slice(block);
		}
		Ok(image.data)
	}

	/// Decodes the PIXLZR image from a vector of bytes, following the steps:
	/// 1. Extract header
	/// 2. Get line lengths
	/// 3.
	pub fn decode_from_vec(inp: Raw) -> Result_QOI<Self> {
		let mut reader = Bytes::new(inp);

		// Get header info
		assert_eq!(
			PIXLZR_MAGIC_NUMBERS,
			reader
				.read_slice(PIXLZR_MAGIC_NUMBERS.len())
				.expect(ERR_ENDED_TOO_SOON)
		);
		let version: Semver =
			reader.read_slice(3).expect(ERR_ENDED_TOO_SOON).into();
		let mut filter = None;

		if has_resource(&version, "filter") {
			filter = Some(reader.read_u8().into());
		}

		let width = reader.read_u32();
		let height = reader.read_u32();
		let block_width = reader.read_u32();
		let block_height = reader.read_u32();

		let cols = (width as f32 / block_width as f32).ceil() as usize;
		let rows = (height as f32 / block_height as f32).ceil() as usize;

		// Get the length of each line of blocks
		// In the form `(start, end)[]`
		let line_positions: Vec<(usize, usize)> = {
			let line_sizes: Vec<u32> =
				(0..rows).map(|_| reader.read_u32()).collect();

			let line_positions: Vec<(usize, usize)> = line_sizes
				.iter()
				.scan(reader.index(), |sum, x| {
					let old_sum = *sum;
					let x = *x as usize;
					*sum += x;
					Some((old_sum, *sum))
				})
				.collect();

			line_positions
		};

		// Decode and collect the blocks
		assert_eq!(reader.data.len(), line_positions.last().unwrap().1);
		let blocks: Vec<_> = line_positions
			.iter()
			.flat_map(|&(start, end)| {
				// Create a view
				let mut view = bytes_cutout!(reader[start..end]);

				// For each block
				(0..cols)
					.map(|_| {
						PixlzrBlock::from(decode_block(&mut view).unwrap())
					})
					.collect::<Vec<_>>()
			})
			.collect::<Vec<_>>();

		Ok(Self {
			width,
			height,
			block_width,
			block_height,
			blocks,
			filter,
		})
	}
}

fn encode_block(block: &crate::PixlzrBlock) -> (Bytes, usize) {
	let mut output = Bytes::new(vec![0; PIXLZR_BLOCK_HEADER_BASE_SIZE]);

	// Writes PIXLZR_BLOCK magic numbers
	output.write_slice(PIXLZR_BLOCK_MAGIC_NUMBERS);
	output.write_f32(if let Some(value) = block.block_value() {
		value
	} else {
		// TODO: deal with an absent block value
		0.
	});

	// Create an QOI Encoder
	let encoder = {
		let data = block.as_slice();
		let (width, height) = block.dimensions();
		qoi::Encoder::new(data, width, height)
	}
	.unwrap();

	// Encode in the QOI format
	let encoded = encoder.encode_to_vec().unwrap();
	// Discards the QOI magic numbers
	let encoded = &encoded[QOI_MAGIC_SIZE..];

	let len = encoded.len();
	// Writes the length of the QOI block
	output.write_u32(len as u32);
	// Writes the QOI block
	output.write_slice(encoded);

	(output, PIXLZR_BLOCK_HEADER_BASE_SIZE + len)
}

#[cfg(feature = "image-rs")]
fn decode_block(reader: &mut Bytes) -> Result_QOI<PixlzrBlockImage> {
	// Checks for the header's magic numbers
	assert_eq!(
		PIXLZR_BLOCK_MAGIC_NUMBERS,
		reader
			.read_slice(PIXLZR_BLOCK_MAGIC_NUMBERS.len())
			.expect(ERR_ENDED_TOO_SOON)
	);

	// Get block value
	let block_value = reader.read_f32();
	// Get block length
	let len = reader.read_u32();

	// Gets QOI buffer
	let encoded = {
		let mut data = QOI_MAGIC.to_vec();
		data.extend_from_slice(
			reader.read_slice(len as usize).expect(ERR_ENDED_TOO_SOON),
		);
		data
	};

	// Decodes QOI block
	let (qoi_header, qoi_data) = qoi::decode_to_vec(encoded)?;

	// Gets image data
	let width = qoi_header.width;
	let height = qoi_header.height;
	let data = if qoi_header.channels.is_rgba() {
		RgbaImage::from_vec(width, height, qoi_data).unwrap().into()
	} else {
		RgbImage::from_vec(width, height, qoi_data).unwrap().into()
	};

	Ok(PixlzrBlockImage {
		width,
		height,
		data,
		block_value: Some(block_value),
	})
}
