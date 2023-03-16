use crate::{data_types::PixlzrBlock, operations::*, split::split_image};
use image::{imageops::FilterType, DynamicImage, GenericImage, GenericImageView};

///
///
///
///
/// If threshold is negative, invert the operation
pub fn process_custom<F0, F1>(
    image: &DynamicImage,
    threshold: f32,
    block_width: u32,
    block_height: u32,
    mut min_block_width: u32,
    mut min_block_height: u32,
    filter_downscale: FilterType,
    filter_upscale: FilterType,
    before_average: &F0,
    after_average: &F1,
) -> DynamicImage
where
    F0: Fn(f32, f32) -> f32,
    F1: Fn(f32) -> f32,
{
    min_block_width = min_block_width.max(4);
    min_block_height = min_block_height.max(4);
    if block_width <= min_block_width || block_height <= min_block_height {
        return image.clone();
    }
    let is_positive = threshold >= 0.0;
    let threshold = threshold.abs();
    // New image
    let mut output = DynamicImage::new_rgba8(image.width(), image.height());
    // For each splitten block
    for section in split_image(&image, block_width, block_height) {
        // Get the block and it's dimensions
        let block: DynamicImage = match section.block {
            PixlzrBlock::Image(section) => section.data,
            _ => panic!(),
        };
        let (x, y) = (section.x, section.y);
        let (w0, h0) = block.dimensions();
        // Calculate the value
        let value = get_block_variance(&block, &before_average, &after_average);
        // Post-process the value
        let img = if (value >= threshold) ^ is_positive {
            // Calculate the resize level and dimensions
            reduce_image_section(value, &block, filter_downscale)
                .data
                .resize(w0, h0, filter_upscale)
        } else {
            process_custom(
                &block,
                threshold,
                block_width >> 1,
                block_height >> 1,
                min_block_width,
                min_block_height,
                filter_downscale,
                filter_upscale,
                before_average,
                after_average,
            )
        };
        // Saves it's data in the output buffer
        output.copy_from(&img, x, y).unwrap();
    }
    // Returns the new image
    output
}

///
///
///
///
pub fn process(image: &DynamicImage, block_size: u32, threshold: f32) -> DynamicImage {
    let before_average = |x: f32, avg: f32| (x - avg).abs();
    // |x, avg| (x - avg).pow(2)
    let after_average = |x: f32| x;
    // |x| x.sqrt()

    process_custom(
        image,
        threshold,
        block_size,
        block_size,
        4,
        4,
        FilterType::Lanczos3,
        FilterType::Nearest,
        &before_average,
        &after_average,
    )
}

// /
// /
// /
// /
// pub fn process_full<F0>(
//     image: &DynamicImage,
//     threshold: f32,
//     fn_variance: Option<F0>,
// ) -> DynamicImage
// where
//     F0: (Fn(f32) -> f32) + Copy,
// {
//     if let Some(_fn_variance) = fn_variance {
//         return process_custom(
//             image,
//             threshold,
//             image.width(),
//             image.height(),
//             4,
//             4,
//             FilterType::Lanczos3,
//             FilterType::Nearest,
//             _fn_variance,
//         );
//     }
//     let fn_variance = |x: f32| x;

//     process_custom(
//         image,
//         threshold,
//         image.width(),
//         image.height(),
//         4,
//         4,
//         FilterType::Lanczos3,
//         FilterType::Nearest,
//         fn_variance,
//     )
// }
