use crate::{data_types::Pixlzr, PixlzrBlock, PixlzrBlockImage};
use image::{RgbImage, RgbaImage};
use qoi;

type Raw = Vec<u8>;

pub const PIXLZR_MAGIC_NUMBERS: &[u8] = b"PIXLZR";
pub const PIXLZR_MAGIC_VERSION: &[u8] = &[0, 0, 1];
pub const PIXLZR_BLOCK_HEADER: &[u8] = b"block";
pub const PIXLZR_HEADER_SIZE: usize =
    PIXLZR_MAGIC_NUMBERS.len() + PIXLZR_MAGIC_VERSION.len() + 4 * 4;

#[inline]
fn encode_u32(out: &mut Raw, index: usize, number: u32) -> usize {
    let index2 = index + 4;
    out[index..index2].copy_from_slice(&number.to_be_bytes());
    index2
}
#[inline]
fn encode_f32(out: &mut Raw, index: usize, number: f32) -> usize {
    let index2 = index + 4;
    out[index..index2].copy_from_slice(&number.to_be_bytes());
    index2
}
#[inline]
fn encode_slice(out: &mut Raw, index: usize, slice: &[u8]) -> usize {
    // println!("{:#?}[{}] <- {:?}", out, index, slice);
    let len = slice.len();
    let index2 = index + len;
    out.reserve(len);
    out[index..index2].copy_from_slice(slice);
    index2
}
#[inline]
fn decode_u32(inp: &Raw, index: &mut usize) -> u32 {
    let indexx = *index;
    let number = u32::from_be_bytes([
        inp[indexx],
        inp[indexx + 1],
        inp[indexx + 2],
        inp[indexx + 3],
    ]);
    *index += 4;
    number
}
#[inline]
fn decode_f32(inp: &Raw, index: &mut usize) -> f32 {
    let indexx = *index;
    let number = f32::from_be_bytes([
        inp[indexx],
        inp[indexx + 1],
        inp[indexx + 2],
        inp[indexx + 3],
    ]);
    *index += 4;
    number
}
#[inline]
fn decode_slice<'a>(inp: &'a Raw, index: &mut usize, len: usize) -> &'a [u8] {
    let indexx = *index;
    // println!("<>[{}..{}]", indexx, len);
    let slice = &inp[indexx..(indexx + len)];
    *index += len;
    slice
}

impl Pixlzr {
    pub fn encode_to_vec(&self) -> qoi::Result<Raw> {
        let mut out =
            vec![0_u8; self.blocks.len() * self.block_width as usize * self.block_height as usize];

        // Writing
        let mut index: usize = 0;
        // - Header
        index = encode_slice(&mut out, index, PIXLZR_MAGIC_NUMBERS);
        index = encode_slice(&mut out, index, PIXLZR_MAGIC_VERSION);
        index = encode_u32(&mut out, index, self.width);
        index = encode_u32(&mut out, index, self.height);
        index = encode_u32(&mut out, index, self.block_width);
        index = encode_u32(&mut out, index, self.block_height);

        // - Data
        for block in &self.blocks {
            let (width, height) = block.dimensions();
            let data = match block {
                PixlzrBlock::Image(image) => image.data.as_bytes(),
                PixlzrBlock::Raw(raw) => raw.data.as_slice(),
            };
            index = encode_slice(&mut out, index, PIXLZR_BLOCK_HEADER);
            index = encode_f32(&mut out, index, block.block_value().unwrap());
            let encoded = qoi::encode_to_vec(data, width, height)?;
            index = encode_u32(&mut out, index, encoded.len() as u32);
            index = encode_slice(&mut out, index, encoded.as_slice());
        }

        Ok(out)
    }
    pub fn encode_to_vec_vec(&self) -> qoi::Result<Raw> {
        let mut total = vec![];

        // Writing
        let mut index: usize = 0;
        // - Header
        let mut out = vec![0; PIXLZR_HEADER_SIZE];
        index = encode_slice(&mut out, index, PIXLZR_MAGIC_NUMBERS);
        // println!("{:?} @ [{}]", out, index);
        index = encode_slice(&mut out, index, PIXLZR_MAGIC_VERSION);
        index = encode_u32(&mut out, index, self.width);
        index = encode_u32(&mut out, index, self.height);
        index = encode_u32(&mut out, index, self.block_width);
        encode_u32(&mut out, index, self.block_height);
        total.push(out);

        // - Data
        for block in &self.blocks {
            out = vec![0; PIXLZR_BLOCK_HEADER.len() + 8];
            index = 0;
            let (width, height) = block.dimensions();
            let data = match block {
                PixlzrBlock::Image(image) => image.data.as_bytes(),
                PixlzrBlock::Raw(raw) => raw.data.as_slice(),
            };
            index = encode_slice(&mut out, index, PIXLZR_BLOCK_HEADER);
            index = encode_f32(&mut out, index, block.block_value().unwrap());
            let encoder = qoi::Encoder::new(data, width, height)?;
            let encoded = match encoder.encode_to_vec() {
                qoi::Result::Ok(data) => data,
                qoi::Result::Err(err) => panic!("{:#?}", err),
            };
            encode_u32(&mut out, index, encoded.len() as u32);
            total.push(out);
            total.push(encoded);
        }

        Ok(total.concat())
    }
    pub fn decode_from_vec(inp: &Raw) -> Result<Self, qoi::Error> {
        // print!("First d");
        // for i in 0..(inp.len().min(10)) {
        //     print!(" - {:?}", inp.get(i).unwrap());
        // }
        // println!("");
        // Reading
        let mut index: usize = 0;
        // - Header
        assert_eq!(PIXLZR_MAGIC_NUMBERS, decode_slice(inp, &mut index, 6));
        assert_eq!(PIXLZR_MAGIC_VERSION, decode_slice(inp, &mut index, 3));
        let width = decode_u32(inp, &mut index);
        let height = decode_u32(inp, &mut index);
        let block_width = decode_u32(inp, &mut index);
        let block_height = decode_u32(inp, &mut index);

        let horizontal_blocks = (width as f32 / block_width as f32).ceil() as usize;
        let vertical_blocks = (height as f32 / block_height as f32).ceil() as usize;

        // - Blocks
        let mut blocks = vec![];
        blocks.reserve(horizontal_blocks * vertical_blocks);
        while inp.len() > index {
            // - Header
            assert_eq!(decode_slice(inp, &mut index, 5), PIXLZR_BLOCK_HEADER);
            // - Block value
            let block_value = decode_f32(inp, &mut index);
            // - Size
            let len = decode_u32(inp, &mut index);
            // - Data
            let encoded = decode_slice(inp, &mut index, len as usize);
            let (qoi_header, qoi_data) = qoi::decode_to_vec(encoded)?;
            let width = qoi_header.width;
            let height = qoi_header.height;
            let data = if qoi_header.channels.is_rgba() {
                RgbaImage::from_vec(width, height, qoi_data).unwrap().into()
            } else {
                RgbImage::from_vec(width, height, qoi_data).unwrap().into()
            };
            let block = PixlzrBlockImage {
                width,
                height,
                data,
                block_value: Some(block_value),
            };
            blocks.push(block.into());
        }
        Ok(Self {
            width,
            height,
            block_width,
            block_height,
            blocks,
        })
    }
}
