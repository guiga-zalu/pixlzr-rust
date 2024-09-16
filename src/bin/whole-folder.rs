// TODO: Conferir via clippy
#![allow(
	clippy::all,
	clippy::must_use_candidate,
	clippy::cast_sign_loss,
	clippy::cast_precision_loss,
	clippy::cast_possible_truncation,
	clippy::module_name_repetitions
)]
// #![allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
use anyhow::Result;
#[cfg(feature = "image-rs")]
use image::open;
use pixlzr::{FilterType, Pixlzr};
use std::fs;

mod path {
	use core::ops::{Add, Div};
	use std::path::{Path, PathBuf};

	#[derive(Debug, Clone)]
	pub struct ZPath {
		pub buf: PathBuf,
	}
	impl ZPath {
		pub fn new(path: &str) -> Self {
			Self {
				buf: PathBuf::from(path),
			}
		}
	}
	impl AsRef<Path> for ZPath {
		fn as_ref(&self) -> &Path {
			self.buf.as_path()
		}
	}
	impl<P: AsRef<Path>> Add<P> for ZPath {
		type Output = ZPath;
		fn add(self, rhs: P) -> Self::Output {
			Self::Output {
				buf: self.buf.join(rhs),
			}
		}
	}
	#[allow(clippy::suspicious_arithmetic_impl)]
	#[allow(clippy::suspicious_arithmetic_impl)]
	impl<P: AsRef<Path>> Div<P> for ZPath {
		type Output = ZPath;
		fn div(self, rhs: P) -> Self::Output {
			self + rhs
		}
	}
	impl<P: AsRef<Path>> Add<P> for &ZPath {
		type Output = ZPath;
		fn add(self, rhs: P) -> Self::Output {
			(*self).clone() + rhs
		}
	}
	#[allow(clippy::suspicious_arithmetic_impl)]
	impl<P: AsRef<Path>> Div<P> for &ZPath {
		type Output = ZPath;
		fn div(self, rhs: P) -> Self::Output {
			(*self).clone() + rhs
		}
	}
}
use path::ZPath;

fn main() -> Result<()> {
	let base_folder: &ZPath = &ZPath::new("./tests/");
	let images_folder: ZPath = base_folder / "images/";

	let mut entries: Vec<_> = fs::read_dir(images_folder)?
		.map(|res| res.map(|e| e.path()))
		.filter_map(Result::ok)
		.collect();

	entries.sort();
	println!("Folder readden and sorted!");

	// For each parameter
	let block_size = 64;
	for i in 1..21 {
		// if i < 10 {
		//     continue;
		// }
		// Get the shrinking factor
		let k = i as f32 / 20.0;
		let test_name = format!("bs{}x-{}", block_size, (100.0 * k) as u8);
		// Get the test folders
		let pix_folder: ZPath = base_folder / "pix" / &test_name;
		let out_folder: ZPath = base_folder / "out" / &test_name;
		// Assure dirs
		fs::create_dir_all(&pix_folder)?;
		fs::create_dir_all(&out_folder)?;

		println!(
			"Folders {:?} and {:?} assured for parameters (bs = {}, k = {})",
			&pix_folder, &out_folder, block_size, k
		);
		// For each image
		for path_in in entries.clone() {
			let file_stem = path_in.file_stem().unwrap().to_str().unwrap();
			let path_pix = &pix_folder / format!("{file_stem}.pixlzr");
			let path_out = &out_folder / format!("{file_stem}.png");
			each_image(
				path_in.to_str().unwrap(),
				path_pix.buf.to_str().unwrap(),
				path_out.buf.to_str().unwrap(),
				k,
				block_size,
			);
		}
	}

	Ok(())
}

#[inline]
fn each_image(
	path_in: &str,
	path_pix: &str,
	path_out: &str,
	factor: f32,
	block_size: u32,
) {
	// println!("Reading [{}] -> [{}]", path_in, path_pix);
	let _pix = &match write(path_in, path_pix, factor, block_size) {
		Ok(pix) => pix,
		Err(err) => panic!("{err:?}"),
	};
	// println!("Reading [{}] -> [{}]", path_pix, path_out);
	match read(path_pix, path_out /* , pix */) {
		Ok(()) => (),
		Err(err) => panic!("{err:?}"),
	};
}
#[inline]
fn write(
	path_in: &str,
	path_out: &str,
	factor: f32,
	block_size: u32,
) -> Result<Pixlzr> {
	let img = open(path_in)?;
	let mut pix = Pixlzr::from_image(&img, block_size, block_size);

	pix.shrink_by(FilterType::Nearest, factor);

	pix.save(path_out)?;
	Ok(pix)
}

#[inline]
fn read(
	path_in: &str,
	path_out: &str, /* , pix: &Pixlzr */
) -> Result<()> {
	let pix = Pixlzr::open(path_in)?;
	let img = pix.to_image(FilterType::Nearest);
	img.save(path_out)?;
	Ok(())
}
