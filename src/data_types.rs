use crate::{
    operations::{get_block_variance, get_block_variance_by_directions, reduce_image_section},
    split::get_image_block,
};
use image::{
    imageops::FilterType, DynamicImage, GenericImage, GenericImageView, RgbImage, RgbaImage,
};

#[derive(Clone)]
pub struct Pixlzr {
    pub width: u32,
    pub height: u32,
    pub block_width: u32,
    pub block_height: u32,
    pub blocks: Vec<PixlzrBlock>,
    pub filter: Option<FilterType>,
}

impl Pixlzr {
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
    pub fn block_dimensions(&self) -> (u32, u32) {
        (self.block_width, self.block_height)
    }
    pub fn from_image(image: &DynamicImage, block_width: u32, block_height: u32) -> Pixlzr {
        let blocks: Vec<_> = ImageBlockIterator::new(image, block_width, block_height).collect();
        Self {
            width: image.width(),
            height: image.height(),
            block_width,
            block_height,
            blocks,
            filter: None,
        }
    }
    pub fn expand(&self, filter: FilterType) -> Self {
        // println!("Expanding");
        // Extract properties
        let (width, height) = self.dimensions();
        let (block_width, block_height) = self.block_dimensions();
        // Create list of blocks to be returned
        let mut blocks: Vec<_> = vec![];

        let trailing_horizontal = width % block_width;
        let trailing_vertical = height % block_height;
        let cols = width / block_width;
        let rows = height / block_height;

        let mut x = 0;
        let mut y = 0;

        // For each block
        for block in &self.blocks {
            // Reduce the cumulatime limit
            let nwidth = if x == cols {
                trailing_horizontal
            } else {
                block_width
            };
            let nheight = if y == rows {
                trailing_vertical
            } else {
                block_height
            };
            // Extract it's image
            let img = block.as_image().unwrap().data.clone();
            // if y == 0 {
            //     let xx = x * block_width;
            //     let xy = y * block_height;
            //     println!(
            //         "{:?} => {nwidth} x {nheight}\t{x} x {y} <-> {} x {}",
            //         img.dimensions(),
            //         xx + nwidth,
            //         xy + nheight,
            //     );
            // }
            let pix_img = PixlzrBlockImage {
                width: nwidth,
                height: nheight,
                data: if img.width() != nwidth && img.height() != nheight {
                    img.resize_exact(nwidth, nheight, filter)
                } else {
                    img
                },
                block_value: block.block_value(),
            };
            blocks.push(pix_img.into());
            x += 1;
            if (x == cols && trailing_horizontal == 0) || x == cols + 1 {
                x = 0;
                y += 1;
            }
        }
        Self {
            width: self.width,
            height: self.height,
            block_width,
            block_height,
            blocks,
            filter: Some(filter),
        }
    }
    pub fn shrink<F0, F1>(
        &mut self,
        filter_downscale: FilterType,
        before_average: F0,
        after_average: F1,
    ) where
        F0: Fn(f32, f32) -> f32,
        F1: Fn(f32) -> f32,
    {
        self.blocks = self
            .blocks
            .iter()
            .map(|block| {
                if block.block_value().is_none() {
                    let block = block.as_image().unwrap();
                    let img = &block.data;
                    // Calculate the value
                    let value = get_block_variance(img, &before_average, &after_average);
                    let reduced = reduce_image_section((value, value), &img, filter_downscale);
                    // assert_eq!(reduced.width, reduced.data.width());
                    // assert_eq!(reduced.height, reduced.data.height());
                    reduced.into()
                } else {
                    (*block).clone()
                }
            })
            .collect();
    }
    pub fn shrink_by(&mut self, filter_downscale: FilterType, factor: f32) {
        let before_average = |x: f32, avg: f32| (x - avg).abs();
        let after_average = |x: f32| x * factor;
        self.blocks = self
            .blocks
            .iter()
            .map(|block| {
                let block = block.as_image().unwrap();
                let img = &block.data;
                // Calculate the value
                let value = get_block_variance(img, &before_average, &after_average);
                let reduced = reduce_image_section((value, value), &img, filter_downscale);
                // assert_eq!(reduced.width, reduced.data.width());
                // assert_eq!(reduced.height, reduced.data.height());
                reduced.into()
            })
            .collect();
    }
    pub fn shrink_directionally(&mut self, filter_downscale: FilterType, factor: f32) {
        self.blocks = self
            .blocks
            .iter()
            .map(|block| {
                let block = block.as_image().unwrap();
                let img = &block.data;
                // Calculate the value
                let value = get_block_variance_by_directions(img);
                let reduced = reduce_image_section(
                    (value.0 * factor, value.1 * factor),
                    &img,
                    filter_downscale,
                );
                // assert_eq!(reduced.width, reduced.data.width());
                // assert_eq!(reduced.height, reduced.data.height());
                reduced.into()
            })
            .collect();
    }
    pub fn to_image(&self, filter: FilterType) -> DynamicImage {
        // println!("Pre-expansion");
        let pix = self.expand(filter);
        // println!("Post-expansion");
        let mut output = if pix.blocks.iter().any(|block| block.has_alpha()) {
            DynamicImage::new_rgba8(self.width, self.height)
        } else {
            DynamicImage::new_rgb8(self.width, self.height)
        };
        let (block_width, block_height) = pix.block_dimensions();
        let horizontal_blocks = (self.width as f32 / block_width as f32).ceil() as u32;
        let mut x = 0;
        let mut y = 0;
        // println!(
        //     "b wh: {:?}, out wh: {:?}",
        //     pix.block_dimensions(),
        //     pix.dimensions()
        // );
        for block in pix.blocks {
            let img = &block.as_image().unwrap().data;
            // println!(
            //     "xy: ({x}, {y}),\t{:?} => ({}, {})\tim wh: {:?}",
            //     (x * block_width, y * block_height),
            //     x * block_width + img.width(),
            //     y * block_height + img.height(),
            //     img.dimensions(),
            // );
            output
                .copy_from(img, x * block_width, y * block_height)
                .unwrap();
            x += 1;
            if x == horizontal_blocks {
                x = 0;
                y += 1;
            }
        }
        output
    }
}

impl From<Pixlzr> for DynamicImage {
    fn from(value: Pixlzr) -> Self {
        value.to_image(value.filter.unwrap_or(FilterType::Gaussian))
    }
}

/// Image block representation, with:
/// - `x: u32, y: u32` as the coordinates of the block
/// - `block: DynamicImage` as the sub-image
pub struct ImageBlockIterator<'a> {
    width: u32,
    height: u32,
    image: &'a DynamicImage,
    horizontal_blocks: u32,
    vertical_blocks: u32,
    x: u32,
    y: u32,
}

impl<'a> ImageBlockIterator<'a> {
    #[inline]
    pub fn new(image: &'a DynamicImage, width: u32, height: u32) -> Self {
        let (image_width, image_height) = image.dimensions();
        Self {
            width,
            height,
            image,
            horizontal_blocks: (image_width as f32 / width as f32).ceil() as u32,
            vertical_blocks: (image_height as f32 / height as f32).ceil() as u32,
            x: 0,
            y: 0,
        }
    }
    #[inline]
    fn get_block(&self, x: u32, y: u32) -> PixlzrBlock {
        if x > self.horizontal_blocks || y > self.vertical_blocks {
            panic!("Pânico!");
        }
        let width = self.width;
        let height = self.height;
        get_image_block(self.image, x * width, y * height, width, height)
    }
}

impl<'a> Iterator for ImageBlockIterator<'a> {
    type Item = PixlzrBlock;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x == self.horizontal_blocks {
            self.x = 0;
            self.y += 1;
        }
        if self.y == self.vertical_blocks {
            None
        } else {
            let block = self.get_block(self.x, self.y);
            self.x += 1;
            Some(block)
        }
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.len();
        (size, Some(size))
    }
}

impl<'a> ExactSizeIterator for ImageBlockIterator<'a> {
    #[inline]
    fn len(&self) -> usize {
        self.horizontal_blocks as usize * self.vertical_blocks as usize
    }
}

#[derive(Clone)]
pub enum PixlzrBlock {
    Raw(PixlzrBlockRaw),
    Image(PixlzrBlockImage),
}

#[derive(Clone)]
pub struct PixlzrBlockRaw {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub alpha: bool,
    pub block_value: Option<f32>,
}

#[derive(Clone)]
pub struct PixlzrBlockImage {
    pub width: u32,
    pub height: u32,
    pub data: DynamicImage,
    pub block_value: Option<f32>,
}

impl From<PixlzrBlockRaw> for PixlzrBlock {
    fn from(value: PixlzrBlockRaw) -> Self {
        PixlzrBlock::Raw(value)
    }
}
impl From<PixlzrBlockImage> for PixlzrBlock {
    fn from(value: PixlzrBlockImage) -> Self {
        PixlzrBlock::Image(value)
    }
}

impl From<PixlzrBlock> for PixlzrBlockImage {
    fn from(value: PixlzrBlock) -> Self {
        if let PixlzrBlock::Image(image) = value {
            return image;
        }
        let raw = value.as_raw().unwrap();
        let (width, height) = value.dimensions();
        let data = raw.data.to_vec();
        let data: DynamicImage = if raw.alpha {
            RgbaImage::from_raw(width, height, data).unwrap().into()
        } else {
            RgbImage::from_raw(width, height, data).unwrap().into()
        };
        Self {
            width,
            height,
            data,
            block_value: value.block_value(),
        }
    }
}

impl PixlzrBlock {
    pub fn as_image(&self) -> Option<&PixlzrBlockImage> {
        match self {
            PixlzrBlock::Image(image) => Some(image),
            _ => None,
        }
    }
    pub fn as_raw(&self) -> Option<&PixlzrBlockRaw> {
        match self {
            PixlzrBlock::Raw(raw) => Some(raw),
            _ => None,
        }
    }
    pub fn width(&self) -> u32 {
        match self {
            PixlzrBlock::Image(block) => (*block).width,
            PixlzrBlock::Raw(block) => (*block).width,
        }
    }
    pub fn height(&self) -> u32 {
        match self {
            PixlzrBlock::Image(block) => (*block).height,
            PixlzrBlock::Raw(block) => (*block).height,
        }
    }
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width(), self.height())
    }
    pub fn block_value(&self) -> Option<f32> {
        match self {
            PixlzrBlock::Image(block) => (*block).block_value,
            PixlzrBlock::Raw(block) => (*block).block_value,
        }
    }
    pub fn is_image(&self) -> bool {
        match self {
            PixlzrBlock::Image(_) => true,
            _ => false,
        }
    }
    pub fn is_raw(&self) -> bool {
        match self {
            PixlzrBlock::Raw(_) => true,
            _ => false,
        }
    }
    pub fn has_alpha(&self) -> bool {
        match self {
            PixlzrBlock::Raw(raw) => raw.alpha,
            PixlzrBlock::Image(img) => img.data.color().has_alpha(),
        }
    }
    pub fn block_value_was_calculated(&self) -> bool {
        self.block_value().is_some()
    }
}

/// Image block representation, with:
/// - `x: u32, y: u32` as the coordinates of the block
/// - `block: PixlzrBlock` as the sub-image
pub struct ImageBlock {
    pub x: u32,
    pub y: u32,
    pub block: PixlzrBlock,
}

pub mod semver {
    use std::cmp::Ordering;

    #[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
    pub struct Semver {
        pub major: u32,
        pub minor: u32,
        pub patch: u32,
    }

    impl Ord for Semver {
        fn cmp(&self, other: &Self) -> Ordering {
            let m = (&self.major).cmp(&other.major);
            if m.is_eq() {
                let m = (&self.minor).cmp(&other.minor);
                if m.is_eq() {
                    (&self.patch).cmp(&other.patch)
                } else {
                    m
                }
            } else {
                m
            }
        }
    }

    impl PartialOrd for Semver {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl From<&[u8]> for Semver {
        fn from(value: &[u8]) -> Self {
            let len = value.len();
            let mut ver = Semver::default();
            if len > 0 {
                ver.major = value[0] as u32;
                if len > 1 {
                    ver.minor = value[1] as u32;
                    if len > 2 {
                        ver.patch = value[2] as u32;
                    }
                }
            }
            ver
        }
    }
}

pub use semver::Semver;
