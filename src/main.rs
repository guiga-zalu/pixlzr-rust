#![allow(unused)]
use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use image::{imageops::FilterType, open};
use pixlzr::Pixlzr;
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, PartialEq, ValueEnum)]
pub enum CliFilterType {
    /// Nearest Neighbor
    Nearest,

    /// Linear Filter
    Triangle,

    /// Cubic Filter
    CatmullRom,

    /// Gaussian Filter
    Gaussian,

    /// Lanczos with window 3
    Lanczos3,
}

impl From<CliFilterType> for FilterType {
    fn from(value: CliFilterType) -> Self {
        match value {
            CliFilterType::Nearest => FilterType::Nearest,
            CliFilterType::Triangle => FilterType::Triangle,
            CliFilterType::CatmullRom => FilterType::CatmullRom,
            CliFilterType::Gaussian => FilterType::Gaussian,
            CliFilterType::Lanczos3 => FilterType::Lanczos3,
        }
    }
}

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
        default_value_t = ("1".to_string()),
        allow_hyphen_values = true
    )]
    shrinking_factor: String,
    /// The filter used when resizing the image blocks
    #[arg(short, long, value_enum, default_value_t = CliFilterType::Lanczos3)]
    filter: CliFilterType,
    /// Direction-wise scan
    #[arg(short, long)]
    direction_wise: Option<bool>,
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

    (if is_negative { -1f32 } else { 1f32 }) * (if invert { 1f32 / factor } else { factor })
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
        Operation { from, to }
    }
}
fn run(Operation { from, to }: Operation, args: CliArgs, shrink_by: f32) -> Result<()> {
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
const PIX_CVRT_ERROR: &'static str = "Could not convert to an image";

#[inline]
fn format_file_error<'a>(base: &str, file: &PathBuf) -> String {
    format!("{} [ {} ]", base, file.to_str().unwrap())
}

#[inline]
fn image_to_pix(
    CliArgs {
        input,
        output,
        block_width,
        block_height,
        filter,
        direction_wise,
        shrinking_factor: _,
    }: CliArgs,
    shrink_by: f32,
) -> Result<()> {
    let img = open(&input).with_context(|| format_file_error(IMG_OPEN_ERROR, &input))?;

    let mut pix = Pixlzr::from_image(&img, block_width, block_height.unwrap());
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
#[inline]
fn image_to_image(
    CliArgs {
        input,
        output,
        block_width,
        block_height,
        filter,
        direction_wise,
        shrinking_factor: _,
    }: CliArgs,
    shrink_by: f32,
) -> Result<()> {
    let img = open(&input).with_context(|| format_file_error(IMG_OPEN_ERROR, &input))?;

    let mut pix = Pixlzr::from_image(&img, block_width, block_height.unwrap());
    let filter = filter.into();
    if direction_wise.unwrap() {
        pix.shrink_directionally(filter, shrink_by);
    } else {
        pix.shrink_by(filter, shrink_by);
    }
    let img = pix.to_image(filter);
    img.save(&output)
        .with_context(|| format_file_error(IMG_SAVE_ERROR, &output))?;
    Ok(())
}
#[inline]
fn pix_to_image(args: &CliArgs, shrink_by: f32) -> Result<()> {
    let filter = FilterType::from(args.filter);
    let pix = Pixlzr::open(&args.input)
        .with_context(|| format_file_error(IMG_OPEN_ERROR, &args.input))?;
    let img = pix.to_image(filter);
    img.save(&args.output)
        .with_context(|| format_file_error(IMG_SAVE_ERROR, &args.output))?;
    Ok(())
}
#[inline]
fn pix_to_pix(
    CliArgs {
        input,
        output,
        block_width,
        block_height,
        filter,
        direction_wise,
        shrinking_factor: _,
    }: CliArgs,
    shrink_by: f32,
) -> Result<()> {
    let filter = FilterType::from(filter);
    let mut pix = Pixlzr::from_image(
        &Pixlzr::open(&input)
            .with_context(|| format_file_error(IMG_OPEN_ERROR, &input))?
            .to_image(filter),
        block_width,
        block_height.unwrap(),
    );
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

#[allow(unused)]
mod main_all {
    use anyhow::Result;
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
                &self.buf.as_path()
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
        impl<'a, P: AsRef<Path>> Div<P> for &ZPath {
            type Output = ZPath;
            fn div(self, rhs: P) -> Self::Output {
                (*self).clone() + rhs
            }
        }
    }
    use path::ZPath;

    #[allow(unused)]
    fn main_all() -> Result<()> {
        let base_folder: &ZPath = &ZPath::new("./tests/");
        let images_folder: ZPath = base_folder / "images/";

        let mut entries: Vec<_> = fs::read_dir(images_folder)?
            .map(|res| res.map(|e| e.path()))
            .filter(|e| e.is_ok())
            .map(|e| e.unwrap())
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
    fn each_image(path_in: &str, path_pix: &str, path_out: &str, factor: f32, block_size: u32) {
        // println!("Reading [{}] -> [{}]", path_in, path_pix);
        let _pix = &match write(path_in, path_pix, factor, block_size) {
            Ok(pix) => pix,
            Err(err) => panic!("{:?}", err),
        };
        // println!("Reading [{}] -> [{}]", path_pix, path_out);
        match read(path_pix, path_out /* , pix */) {
            Ok(_) => (),
            Err(err) => panic!("{:?}", err),
        };
    }
    #[inline]
    fn write(path_in: &str, path_out: &str, factor: f32, block_size: u32) -> Result<Pixlzr> {
        let img = open(path_in)?;
        let mut pix = Pixlzr::from_image(&img, block_size, block_size);

        pix.shrink_by(FilterType::Nearest, factor);

        pix.save(path_out)?;
        Ok(pix)
    }
    #[inline]
    fn read(path_in: &str, path_out: &str /* , pix: &Pixlzr */) -> Result<()> {
        let pix = Pixlzr::open(path_in)?;
        let img = pix.to_image(FilterType::Nearest);
        img.save(path_out)?;
        Ok(())
    }
}

#[allow(unused)]
fn main_tree() {
    use image::{open, DynamicImage};
    fn processar(imagem: &DynamicImage, k: f32) -> DynamicImage {
        use pixlzr::tree::process as tree;
        tree(&imagem, 128, k)
    }
    // fn processar_pasta(pasta: &str) {
    //     use std::fs;
    //     // Ler pasta
    //     let caminhos = fs::read_dir(pasta).expect("Erro ao ler a pasta!");
    //     // Para cada caminho na pasta
    //     for caminho_ in caminhos {
    //         let caminho = caminho_.expect("Erro ao processar o caminho!").path();
    //         let nome: &str = caminho.to_str().unwrap();
    //         println!("{nome}");
    //         // Lê a imagem
    //         let imagem: DynamicImage = open(nome).expect("Arquivo não encontrado!");
    //         // A processa
    //         processar(&imagem)
    //             // E a salva
    //             .save(nome.replacen("img/", "dst/octree/", 1))
    //             .expect("Erro ao salvar");
    //     }
    // }

    // let block_size: u32 = 128;
    // processar_pasta("./img");
    let nome = "./img/blur.jpg";
    let n: u16 = 600;
    // Lê a imagem
    let imagem: DynamicImage = open(nome).expect("Arquivo não encontrado!");
    for i in 0..n {
        let k = i as f32 / n as f32;
        println!("Quadro {i} / {n} ({k} %)");
        let m = format!("0000{i}");
        let len = m.len();
        let n = &m[(len - 4)..len];
        let j = format!("./dst/video/{n}.png");
        // Clona a imagem
        let img = imagem.clone();
        // A processa
        processar(&img, k)
            // E a salva
            .save(j)
            .expect("Erro ao salvar");
    }
}
