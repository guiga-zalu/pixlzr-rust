use anyhow::{Context, Result};
use clap::Parser;
use image::open;
use pixlzr::{FilterType, Pixlzr};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
	// Files
	/// The input image file
	#[arg(short, long)]
	input: PathBuf,
	#[arg(short, long)]
	/// The output image file
	output: PathBuf,
	// Block size
	/// The width of each block
	#[arg(short, long, default_value_t = 64)]
	block_width: u32,
	/// The height of each block
	#[arg(long)]
	block_height: Option<u32>,
	// Shrinker
	/// The shrinking factor: [+|-][1/][D][.D]
	///  If negative, is passed through max(0, 1 - x).
	#[arg(
        short = 'k',
        long,
        default_value_t = String::from("1"),
        allow_hyphen_values = true
    )]
	shrinking_factor: String,
	/// The filter used when resizing the image blocks
	#[arg(short, long, value_enum, default_value_t = FilterType::Lanczos3)]
	filter: FilterType,
	/// Direction-wise scan
	#[arg(short, long)]
	direction_wise: Option<bool>,
	/// If image-2-image, force shrinking?
	#[arg(long, default_value_t = false)]
	force: bool,
}

const DEFAULT_SHRINKING_FACTOR: f32 = 1f32;

fn parse_shrinking_factor(shrinking_factor: &str) -> f32 {
	let mut base_pos: usize = 0;
	let mut invert = false;
	let mut is_negative = false;
	if shrinking_factor[base_pos..].starts_with("+") {
		base_pos += 1;
	} else if shrinking_factor[base_pos..].starts_with("-") {
		is_negative = true;
		base_pos += 1;
	}
	if shrinking_factor[base_pos..].starts_with("1/") {
		invert = true;
		base_pos += 2;
	}

	let factor = shrinking_factor[base_pos..]
		.parse()
		.unwrap_or(DEFAULT_SHRINKING_FACTOR);

	(if is_negative { -1.0 } else { 1.0 })
		* (if invert { 1.0 / factor } else { factor })
}

fn main() -> Result<()> {
	let mut args = CliArgs::parse();
	// println!("{:#?}", args);
	args.block_height = args.block_height.or(Some(args.block_width));
	args.direction_wise = args.direction_wise.or(Some(false));
	let shrink_by = parse_shrinking_factor(args.shrinking_factor.as_str());
	// println!("{shrink_by}");

	run((&args.input, &args.output).into(), args, shrink_by)?;
	Ok(())
}

#[derive(Debug)]
pub enum Type {
	Pix,
	Image,
}

pub struct Operation {
	from: Type,
	to: Type,
}

impl From<(&PathBuf, &PathBuf)> for Operation {
	fn from((input, output): (&PathBuf, &PathBuf)) -> Self {
		let from = if let Some(input_ext) = input.extension() {
			match input_ext.to_ascii_lowercase().to_str().unwrap_or("") {
				"pix" | "pixlzr" => Type::Pix,
				_ => Type::Image,
			}
		} else {
			Type::Image
		};
		let to = if let Some(output_ext) = output.extension() {
			match output_ext.to_ascii_lowercase().to_str().unwrap_or("") {
				"pix" | "pixlzr" => Type::Pix,
				_ => Type::Image,
			}
		} else {
			Type::Pix
		};
		// println!("From: {:?}, To: {:?}", from, to);
		Operation { from, to }
	}
}

fn run(
	Operation { from, to }: Operation,
	args: CliArgs,
	shrink_by: f32,
) -> Result<()> {
	// println!("{:?} -> {:?}", from, to);
	match from {
		Type::Image => match to {
			Type::Pix => image_to_pix(args, shrink_by),
			Type::Image => image_to_image(args, shrink_by),
		},
		Type::Pix => match to {
			Type::Image => pix_to_image(&args, shrink_by),
			Type::Pix => pix_to_pix(args, shrink_by),
		},
	}
}

const IMG_OPEN_ERROR: &'static str = "Could not open the image";
const IMG_SAVE_ERROR: &'static str = "Could not save the image";

#[inline]
fn format_file_error<'a>(base: &str, file: &PathBuf) -> String {
	format!("{} [ {} ]", base, file.to_str().unwrap())
}

fn image_to_pix(
	CliArgs {
		input,
		output,
		block_width,
		block_height,
		filter,
		direction_wise,
		shrinking_factor: _,
		force: _,
	}: CliArgs,
	shrink_by: f32,
) -> Result<()> {
	let img = open(&input)
		.with_context(|| format_file_error(IMG_OPEN_ERROR, &input))?;

	let mut pix =
		Pixlzr::from_image(&img, block_width, block_height.unwrap());

	let filter = filter.into();
	if direction_wise.unwrap() {
		pix.shrink_directionally(filter, shrink_by);
	} else {
		pix.shrink_by(filter, shrink_by);
	}

	pix.save(&output)
		.with_context(|| format_file_error(IMG_SAVE_ERROR, &output))?;
	Ok(())
}

fn image_to_image(
	CliArgs {
		input,
		output,
		block_width,
		block_height,
		filter,
		direction_wise,
		shrinking_factor: _,
		force,
	}: CliArgs,
	shrink_by: f32,
) -> Result<()> {
	let img = open(&input)
		.with_context(|| format_file_error(IMG_OPEN_ERROR, &input))?;

	let mut pix =
		Pixlzr::from_image(&img, block_width, block_height.unwrap());

	let filter = filter.into();
	if force {
		if direction_wise.unwrap() {
			pix.shrink_directionally(filter, shrink_by);
		} else {
			pix.shrink_by(filter, shrink_by);
		}
	}

	let img = pix.to_image(filter);
	img.save(&output)
		.with_context(|| format_file_error(IMG_SAVE_ERROR, &output))?;
	Ok(())
}

fn pix_to_image(args: &CliArgs, shrink_by: f32) -> Result<()> {
	let filter = args.filter;
	let mut pix = Pixlzr::open(&args.input)
		.with_context(|| format_file_error(IMG_OPEN_ERROR, &args.input))?;

	if args.force {
		let filter = filter.into();
		if args.direction_wise.unwrap() {
			pix.shrink_directionally(filter, shrink_by);
		} else {
			pix.shrink_by(filter, shrink_by);
		}
	}

	let img = pix.to_image(filter.into());
	img.save(&args.output).with_context(|| {
		format_file_error(IMG_SAVE_ERROR, &args.output)
	})?;
	Ok(())
}

fn pix_to_pix(
	CliArgs {
		input,
		output,
		block_width,
		block_height,
		filter,
		direction_wise,
		shrinking_factor: _,
		force,
	}: CliArgs,
	shrink_by: f32,
) -> Result<()> {
	let filter = filter.into();
	let mut pix = Pixlzr::from_image(
		&Pixlzr::open(&input)
			.with_context(|| format_file_error(IMG_OPEN_ERROR, &input))?
			.to_image(filter),
		block_width,
		block_height.unwrap(),
	);

	if force {
		let filter = filter.into();
		if direction_wise.unwrap() {
			pix.shrink_directionally(filter, shrink_by);
		} else {
			pix.shrink_by(filter, shrink_by);
		}
	}

	pix.save(&output)
		.with_context(|| format_file_error(IMG_SAVE_ERROR, &output))?;
	Ok(())
}
