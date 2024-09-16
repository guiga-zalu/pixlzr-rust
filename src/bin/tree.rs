#[cfg(feature = "image-rs")]
use image::{open, DynamicImage};
#[cfg(feature = "image-rs")]
fn processar(imagem: &DynamicImage, k: f32) -> DynamicImage {
	use pixlzr::tree::process as tree;
	tree(imagem, 128, k)
}

#[allow(clippy::cast_lossless)]
fn main() {
	// let block_size: u32 = 128;
	// processar_pasta("./img");
	let nome = "./img/blur.jpg";
	let n: u16 = 600;
	// Lê a imagem
	let imagem: DynamicImage =
		open(nome).expect("Arquivo não encontrado!");
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
