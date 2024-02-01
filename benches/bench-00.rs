use criterion::{black_box, criterion_group, criterion_main, Criterion};
use image::{self, DynamicImage};
use pixlzr::{FilterType, Pixlzr};
use std::fs::read as read_file;

type V = Vec<u8>;

#[allow(unused)]
fn decode_pix(data: &V) -> () {
	// let _ = Pixlzr::decode_from_vec(data).unwrap();
	()
}
fn encode_pix(pix: &Pixlzr) -> () {
	pix.encode_to_vec().unwrap();
	()
}
fn decode_png(data: &V) -> () {
	image::load_from_memory_with_format(
		data.as_slice(),
		image::ImageFormat::Png,
	)
	.unwrap();
	()
}
// fn encode_png(img: &DynamicImage) -> () {
//     image::write_buffer_with_format(buffered_writer, buf, width, height, color, format)
// }

fn convert_to_image(img: &DynamicImage) -> () {
	let _ = Pixlzr::from_image(img, 64, 64);
	()
}
fn shrink(pix: &mut Pixlzr) -> () {
	pix.shrink_by(FilterType::CatmullRom, 0.5);
	()
}

pub fn criterion_benchmark(c: &mut Criterion) {
	let path_pix: &str = "./benches/base.pixlzr";
	let path_png: &str = "./benches/base.png";

	let data_pix: V = read_file(&path_pix).unwrap();
	let data_png: V = read_file(&path_png).unwrap();

	let img = image::load_from_memory_with_format(
		&data_png,
		image::ImageFormat::Png,
	)
	.unwrap();
	let mut pix = Pixlzr::from_image(&img, 64, 64);

	c.bench_function("raw decoding pix", |b| {
		b.iter(|| decode_pix(black_box(&data_pix)))
	});
	c.bench_function("raw decoding png", |b| {
		b.iter(|| decode_png(black_box(&data_png)))
	});
	c.bench_function("raw encoding pix", |b| {
		b.iter(|| encode_pix(black_box(&pix)))
	});
	// c.bench_function("raw encoding png", |b| {
	//     b.iter(|| encode_png(black_box(&img)))
	// });
	c.bench_function("raw converting to image", |b| {
		b.iter(|| convert_to_image(black_box(&img)))
	});
	c.bench_function("raw shrinking", |b| {
		b.iter(|| shrink(black_box(&mut (pix.clone()))))
	});

	pix.shrink_by(FilterType::CatmullRom, 1.0);
	c.bench_function("raw encoding pix - shrunk", |b| {
		b.iter(|| encode_pix(black_box(&pix)))
	});
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
