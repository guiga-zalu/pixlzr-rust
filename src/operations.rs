use image::{imageops::FilterType, DynamicImage, GenericImageView, Rgba};

/// Calculates a `[0; 1]` value for the pixel variance of a given `img` image
///
/// 1. Calculates the average of pixel values
/// 2. Calculates the total difference of these values
/// 3. Normalizes the result to `[0; 1]`
pub fn get_block_variance<T, F0, F1>(img: &T, before: &F0, after: &F1) -> f32
where
    T: GenericImageView<Pixel = Rgba<u8>>,
    F0: Fn(f32, f32) -> f32,
    F1: Fn(f32) -> f32,
{
    // 1. Calculates the average of pixel values
    let mut sum: [u32; 4] = [0, 0, 0, 0];
    let mut count: u64 = 0;
    for (_x, _y, pixel) in img.pixels() {
        let rgba = pixel.0;
        sum[0] += rgba[0] as u32;
        sum[1] += rgba[1] as u32;
        sum[2] += rgba[2] as u32;
        sum[3] += rgba[3] as u32;
        count += 1;
    }
    let count = count as f32;
    let average = [
        sum[0] as f32 / count,
        sum[1] as f32 / count,
        sum[2] as f32 / count,
        sum[3] as f32 / count,
    ];
    // 2. Calculates the total difference of these values
    let mut delta: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
    for (_x, _y, pixel) in img.pixels() {
        let rgba = pixel.0;
        delta[0] += before(rgba[0] as f32, average[0]);
        delta[1] += before(rgba[1] as f32, average[1]);
        delta[2] += before(rgba[2] as f32, average[2]);
        delta[3] += before(rgba[3] as f32, average[3]);
    }
    // 3. Normalizes the result to `[0; 1]`
    /*
    - $p_{i, j}$: pixel at position $i, j$
    - $W, H$: image's width and height
    - $M$: maximum subpixel value
    - $\bar p$: average pixel value
    - $\delta_{i, j}$: per pixel difference
    - $\int\delta$: sum of differences
    The maximum value for $\in\delta$ is* when:
    - $p_{i, j} = M$ for half of the values of $i, j$,
    - and $0$ for the other half.
    So $\bar p = {M\over 2}$ and $\delta_{i, j} = {M\over 2}$.
    Thus, $\int\delta = W\cdot H\times \delta_{i, j} = {W\cdot H\cdot M\over 2}$.
    cont := W * H
     */
    let factor = count * (u8::MAX >> 1) as f32;
    after(delta[0] + delta[1] + delta[2] + delta[3]) / factor
}

pub fn reduce_image_section(
    value: f32,
    w0: u32,
    h0: u32,
    block: DynamicImage,
    filter_downscale: FilterType,
    filter_upscale: FilterType,
) -> DynamicImage {
    let level = value.log2().round().min(0f32).exp2();
    let w1 = (w0 as f32 * level).ceil() as u32;
    let h1 = (h0 as f32 * level).ceil() as u32;
    // Resizes the image down and back
    let img = block
        .resize(w1, h1, filter_downscale)
        .resize(w0, h0, filter_upscale);
    img
}
