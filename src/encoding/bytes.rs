#![allow(clippy::uninit_vec)]
use super::Raw;

macro_rules! safe_copy {
	($vec:ident[$index:expr] +=[$len:expr] $slice:ident) => {{
		let new_index = $index + $len;
		// Method 1: resize the vector and initialize with 0s
		// $vec.resize(new_index, 0);
		// Method 2: reserve space (the capacity), and sets the length
		// - It's safe because it guarantees the capacity, and initializes the data
		$vec.reserve($len);
		unsafe { $vec.set_len(new_index) };
		// Copies the data from the slice
		$vec[$index..new_index].copy_from_slice($slice);
		// Returns the new index
		new_index
	}};
	($vec:ident[$index:expr] += $slice:ident) => {{
		let len = $slice.len();
		let new_index = $index + len;
		// Method 1: resize the vector and initialize with 0s
		// $vec.resize(new_index, 0);
		// Method 2: reserve space (the capacity), and sets the length
		// - It's safe because it guarantees the capacity, and initializes the data
		$vec.reserve(len);
		unsafe { $vec.set_len(new_index) };
		// Copies the data from the slice
		$vec[$index..new_index].copy_from_slice($slice);
		// Returns the new index
		new_index
	}};
}

/// The `Bytes` struct represents a byte buffer with additional functionality.
/// It allows for creating new `Bytes` objects with initial data and index values,
/// reserving capacity for new `Bytes` objects, and checking if there are more
/// elements in the byte buffer to be read.
///
/// # Examples
///
/// ```
/// use pixlzr::encoding::bytes::Bytes;
///
/// let bytes = Bytes::new(vec![1, 2, 3]);
/// assert_eq!(bytes.yet_readding(), true);
///
/// let reserved_bytes = Bytes::reserve(10);
/// assert_eq!(reserved_bytes.yet_readding(), false);
/// ```
#[derive(Debug, Default)]
pub struct Bytes {
	pub data: Raw,
	index: usize,
}

#[allow(unused)]
impl Bytes {
	/// Creates a new `Bytes` object with the given `data`, that will be read from 0 onwards.
	pub fn new(data: Raw) -> Self {
		Bytes { data, index: 0 }
	}

	pub fn index(&self) -> usize {
		self.index
	}

	/// Creates a new `Bytes` object with an empty `data` vector,
	///  with a pre-reserved capacity to hold `length` elements.
	pub fn reserve(length: usize) -> Self {
		Self::new(Vec::with_capacity(length))
	}

	/// Creates a new `Bytes` object with an empty `data` vector,
	///  with a pre-reserved capacity to hold *exactly* `length` elements.
	pub fn reserve_exact(length: usize) -> Self {
		let mut data = Vec::new();
		data.reserve_exact(length);
		Self::new(data)
	}

	/// Checks if there are more elements in the byte buffer to be read.
	#[inline]
	pub fn yet_readding(&self) -> bool {
		self.data.len() > self.index
	}

	// /// Returns a view of the byte buffer from the current index to the next `length` octets.
	// pub fn view_with_length(&self, length: usize) -> Self {
	// 	Self::new(&self.data[self.index..self.index + length])
	// }
}

impl From<Raw> for Bytes {
	fn from(value: Raw) -> Self {
		Self::new(value)
	}
}

macro_rules! bytes_cutout {
	($byte:ident[$span:expr]) => {
		Bytes::from($byte.data[$span].to_owned())
	};
}

#[allow(unused)]
/// Readding methods
impl Bytes {
	pub fn read_u8(&mut self) -> u8 {
		let number = self.data[self.index];
		self.index += 1;
		number
	}

	pub fn read_u16(&mut self) -> u16 {
		let number = u16::from_be_bytes([
			self.data[self.index],
			self.data[self.index + 1],
		]);
		self.index += 2;
		number
	}

	pub fn read_u32(&mut self) -> u32 {
		let number = u32::from_be_bytes([
			self.data[self.index],
			self.data[self.index + 1],
			self.data[self.index + 2],
			self.data[self.index + 3],
		]);
		self.index += 4;
		number
	}

	pub fn read_f32(&mut self) -> f32 {
		let number = f32::from_be_bytes([
			self.data[self.index],
			self.data[self.index + 1],
			self.data[self.index + 2],
			self.data[self.index + 3],
		]);
		self.index += 4;
		number
	}

	pub fn read_u64(&mut self) -> u64 {
		let number = u64::from_be_bytes([
			self.data[self.index],
			self.data[self.index + 1],
			self.data[self.index + 2],
			self.data[self.index + 3],
			self.data[self.index + 4],
			self.data[self.index + 5],
			self.data[self.index + 6],
			self.data[self.index + 7],
		]);
		self.index += 4;
		number
	}

	pub fn read_f64(&mut self) -> f64 {
		let number = f64::from_be_bytes([
			self.data[self.index],
			self.data[self.index + 1],
			self.data[self.index + 2],
			self.data[self.index + 3],
			self.data[self.index + 4],
			self.data[self.index + 5],
			self.data[self.index + 6],
			self.data[self.index + 7],
		]);
		self.index += 4;
		number
	}

	pub fn read_slice(&mut self, len: usize) -> Option<&[u8]> {
		// Check if there are at least `len` elements to be readden
		if self.data.len() < self.index + len {
			return None;
		}

		// println!("<>[{}..{}]", self.index, len);
		let slice = self.data[self.index..(self.index + len)].into();
		self.index += len;
		Some(slice)
	}
}

#[allow(unused)]
/// Writting methods
impl Bytes {
	pub fn write_u8(&mut self, number: u8) {
		// let index2 = self.index + 1;
		// self.data[self.index..index2]
		// 	.copy_from_slice(&number.to_be_bytes());
		// self.index = index2;
		let data = &mut self.data;
		let bytes = &[number];
		self.index = safe_copy!(data[self.index] += [1] bytes);
	}

	pub fn write_u16(&mut self, number: u16) {
		// let index2 = self.index + 2;
		// self.data[self.index..index2]
		// 	.copy_from_slice(&number.to_be_bytes());
		// self.index = index2;
		let data = &mut self.data;
		let bytes = &number.to_be_bytes();
		self.index = safe_copy!(data[self.index] += [2] bytes);
	}

	pub fn write_u32(&mut self, number: u32) {
		// let index2 = self.index + 4;
		// self.data[self.index..index2]
		// 	.copy_from_slice(&number.to_be_bytes());
		// self.index = index2;
		let data = &mut self.data;
		let bytes = &number.to_be_bytes();
		self.index = safe_copy!(data[self.index] += [4] bytes);
	}

	pub fn write_f32(&mut self, number: f32) {
		// let index2 = self.index + 4;
		// self.data[self.index..index2]
		// 	.copy_from_slice(&number.to_be_bytes());
		// self.index = index2;
		let data = &mut self.data;
		let bytes = &number.to_be_bytes();
		self.index = safe_copy!(data[self.index] += [4] bytes);
	}

	pub fn write_u64(&mut self, number: u64) {
		// let index2 = self.index + 8;
		// self.data[self.index..index2]
		// 	.copy_from_slice(&number.to_be_bytes());
		// self.index = index2;
		let data = &mut self.data;
		let bytes = &number.to_be_bytes();
		self.index = safe_copy!(data[self.index] += [8] bytes);
	}

	pub fn write_f64(&mut self, number: f64) {
		// let index2 = self.index + 8;
		// self.data[self.index..index2]
		// 	.copy_from_slice(&number.to_be_bytes());
		// self.index = index2;
		let data = &mut self.data;
		let bytes = &number.to_be_bytes();
		self.index = safe_copy!(data[self.index] += [8] bytes);
	}

	pub fn write_slice(&mut self, slice: &[u8]) {
		// let len = slice.len();
		// let index2 = self.index + len;
		// // Method 1: resize the vector and initialize with 0s
		// // self.data.resize(index2, 0);
		// // Method 2: reserve space (the capacity), and sets the length
		// // - It's safe because it guarantees the capacity, and initializes the data
		// self.data.reserve(len);
		// unsafe {
		// 	self.data.set_len(index2);
		// }
		// self.data[self.index..index2].copy_from_slice(slice);
		// self.index = index2;
		let data = &mut self.data;
		self.index = safe_copy!(data[self.index] += slice);
	}
}
