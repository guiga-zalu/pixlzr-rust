pub mod tree;

use crate::{data_types::PixlzrBlock, operations::*, split::split_image};
use image::{imageops::FilterType, DynamicImage, GenericImage, GenericImageView};

///
pub fn process_into_custom<F0, F1>(
    image: &DynamicImage,
    block_width: u32,
    block_height: u32,
    filter_downscale: FilterType,
    filter_upscale: FilterType,
    before_average: F0,
    after_average: F1,
) -> DynamicImage
where
    F0: Fn(f32, f32) -> f32,
    F1: Fn(f32) -> f32,
{
    // New image
    let mut output = DynamicImage::new_rgba8(image.width(), image.height());
    // For each splitten block
    for section in split_image(&image, block_width, block_height) {
        // Get the block and it's dimensions
        let block: DynamicImage = match section.block {
            PixlzrBlock::Image(section) => section.data,
            _ => panic!(),
        };
        let (w0, h0) = block.dimensions();
        // Calculate the value
        let value = get_block_variance(&block, &before_average, &after_average);
        let img = reduce_image_section(value, &block, filter_downscale)
            .data
            .resize(w0, h0, filter_upscale);
        // Saves it's data in the output buffer
        output.copy_from(&img, section.x, section.y).unwrap();
    }
    // Returns the new image
    output
}

///
/// - Uses `difference := sum of { |pixel - average| } for p in pixels`
///
pub fn process_custom<F0, F1>(
    image: &DynamicImage,
    block_width: u32,
    block_height: u32,
    filter_downscale: FilterType,
    filter_upscale: FilterType,
    before_average: F0,
    after_average: F1,
) -> DynamicImage
where
    F0: Fn(f32, f32) -> f32,
    F1: Fn(f32) -> f32,
{
    // New image
    let mut output = DynamicImage::new_rgba8(image.width(), image.height());
    // For each splitten block
    for section in split_image(&image, block_width, block_height) {
        // Get the block and it's dimensions
        let block: DynamicImage = match section.block {
            PixlzrBlock::Image(section) => section.data,
            _ => panic!(),
        };
        let (w0, h0) = block.dimensions();
        // Calculate the value
        let value = get_block_variance(&block, &before_average, &after_average);
        let img = reduce_image_section(value, &block, filter_downscale)
            .data
            .resize(w0, h0, filter_upscale);
        // Saves it's data in the output buffer
        output.copy_from(&img, section.x, section.y).unwrap();
    }
    // Returns the new image
    output
}

///
///
///
pub fn process(image: &DynamicImage, block_size: u32) -> DynamicImage {
    let before_average = |x: f32, avg: f32| (x - avg).abs();
    // |x, avg| (x - avg).pow(2)
    let after_average = |x: f32| x;
    // |x| x.sqrt()
    process_custom(
        image,
        block_size,
        block_size,
        FilterType::Lanczos3,
        FilterType::Nearest,
        before_average,
        after_average,
    )
}
